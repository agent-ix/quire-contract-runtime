// SPDX-License-Identifier: AGPL-3.0-or-later
//! quire-specification/FR-143 records, tuples and finite recursive values over producer-assigned
//! declaration keys.
//!
//! A [`TypeEnvironment`] admits one checked package's closed set of record,
//! tuple and model object-type declarations. It refuses a containment graph
//! whose unnamed-edge or non-escaping-edge subgraph has a cycle, naming that
//! cycle, a `Reference<T>` whose target is not a model object type, and a set,
//! bag or ordered set whose element type bears IEEE values at any depth.
//! Construction checks every field, position and state against its
//! declaration and refuses at the originating component. Values are immutable
//! and cache their `occ`; sharing a contained value (`Rc`) never creates
//! object identity.

use alloc::boxed::Box;
use alloc::collections::{BTreeMap, BTreeSet};
use alloc::rc::Rc;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::fmt;
use core::fmt::Write as _;
use core::mem;

use super::accounting::{Charge, ChargePoint, LimitKind, Meter};
use super::collection::{CollectionKind, CollectionType, CollectionValue};
use super::comparison::{IllTyped, IllTypedCause};
use super::decimal::{Decimal, DecimalType};
use super::enumeration::EnumValue;
use super::ieee::{IeeeValue, IeeeWidth};
use super::integer::{Integer, IntegerInterval};
use super::node::NodeKey;
use super::outcome::{Outcome, Refusal, Stop};
use super::quantity::{Quantity, QuantityUnit};
use super::rational::{Rational, RationalDomain};
use super::reference::ObjectReference;
use super::text::{Text, TextType};

/// A declared complete-V1 value type. Two types are the same type exactly when
/// they are equal, collection bounds included.
#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
// An explicit tag rather than a niche in a payload's `Vec` capacity (IR-286):
// CBMC cannot constant-fold a niche-encoded discriminant read back from the
// heap, so a `ValueType` in a `Vec` or `Box` sends Kani down this type's
// recursive drop glue without bound. Layout only; no behaviour depends on it.
// A `u64` tag, and no inline payload larger than `IntegerInterval`, for the
// reason `Value` gives: an `Int` domain read back from the heap folds only when
// it starts at the payload union's offset 0 and fills the whole union.
#[repr(u64)]
pub enum ValueType {
    /// `Boolean`.
    Boolean,
    /// The unbounded mathematical `Integer`.
    Integer,
    /// A bounded `Int[lo, hi]`.
    Int(IntegerInterval),
    /// A `Rational[n1, n2; d1, d2]` domain.
    Rational(RationalDomain),
    /// An quire-specification/FR-140 `Decimal[lo, hi; smin, smax; mode]`.
    Decimal(DecimalType),
    /// An quire-specification/FR-148 `Float32` or `Float64`.
    Float(IeeeWidth),
    /// An quire-specification/FR-142 quantity in exactly this unit.
    Quantity(QuantityUnit),
    /// An quire-specification/FR-141 `Text[min, max; profile]`.
    Text(TextType),
    /// A member of the enum declaration with this node key.
    Enum(NodeKey),
    /// `Option<T>`: `none` or a present `T`.
    Option(Box<ValueType>),
    /// The record or tuple declaration with this node key.
    Composite(NodeKey),
    /// A bounded collection type `K<T>[min, max]`.
    Collection(Box<CollectionType>),
    /// `Reference<T>` to an object of the model object type with this key.
    Reference(NodeKey),
}

impl ValueType {
    /// `K<element>[bound]`.
    pub fn collection(collection_type: CollectionType) -> Self {
        Self::Collection(Box::new(collection_type))
    }

    /// `Option<payload>`.
    pub fn option(payload: Self) -> Self {
        Self::Option(Box::new(payload))
    }

    /// Whether `value` is a member of this declared type. Composite, option
    /// and collection values carry their declared type, which must be this
    /// type; their contents were admitted at construction.
    pub fn admits(&self, value: &Value) -> bool {
        match (self, value) {
            (Self::Boolean, Value::Boolean(_)) | (Self::Integer, Value::Integer(_)) => true,
            (Self::Int(interval), Value::Integer(integer)) => interval.contains(integer),
            (Self::Rational(domain), Value::Rational(rational)) => domain.contains(rational),
            (Self::Decimal(declared), Value::Decimal(decimal)) => declared.contains(decimal),
            (Self::Float(width), Value::Float(float)) => float.width() == *width,
            (Self::Quantity(unit), Value::Quantity(quantity)) => quantity.unit() == unit,
            (Self::Text(declared), Value::Text(text)) => text.text_type() == declared,
            (Self::Enum(declaration), Value::Enum(member)) => member.declaration() == *declaration,
            (Self::Option(payload), Value::Option(option)) => option.payload_type() == &**payload,
            (Self::Composite(declaration), Value::Composite(composite)) => {
                composite.declaration() == *declaration
            }
            (Self::Collection(declared), Value::Collection(collection)) => {
                collection.collection_type() == &**declared
            }
            (Self::Reference(object_type), Value::Reference(reference)) => {
                reference.object_type() == *object_type
            }
            (
                Self::Boolean
                | Self::Integer
                | Self::Int(_)
                | Self::Rational(_)
                | Self::Decimal(_)
                | Self::Float(_)
                | Self::Quantity(_)
                | Self::Text(_)
                | Self::Enum(_)
                | Self::Option(_)
                | Self::Composite(_)
                | Self::Collection(_)
                | Self::Reference(_),
                _,
            ) => false,
        }
    }
}

/// A completed complete-V1 value. It deliberately has no structural
/// `PartialEq`: equality is the quire-specification/FR-149 relation of
/// [`CheckedEquality::evaluate`](super::CheckedEquality::evaluate).
///
/// `Debug` (below) and `Drop` (further below) are both hand-written and iterative: a value chain
/// nested as deep as a package's declared recursion bound allows must never recurse the host
/// stack to format or to free, because on the governed `thumbv7em-none-eabi` target a stack
/// overflow is silent memory corruption, not a panic (see the module-level invariant in
/// `src/exact/mod.rs`).
#[non_exhaustive]
#[derive(Clone)]
// An explicit tag, for the reason `ValueType` carries one (IR-286): a niche
// discriminant read back from a `Vec<Value>` is not constant-foldable by CBMC.
// A `u64` tag, not `u8`: CBMC folds a payload read back from the heap only when
// it starts at the payload union's offset 0 and fills the whole union, so every
// inline payload is at most `size_of::<Integer>()` (`Rational`, `Decimal` and
// four other types keep their fields behind a `Box` for this reason).
#[repr(u64)]
pub enum Value {
    /// A Boolean.
    Boolean(bool),
    /// A mathematical integer of `Integer` or `Int[..]`.
    Integer(Integer),
    /// An exact reduced rational.
    Rational(Rational),
    /// An exact decimal.
    Decimal(Decimal),
    /// An IEEE bit pattern.
    Float(IeeeValue),
    /// A quantity in its unit.
    Quantity(Quantity),
    /// A text value of its declared type.
    Text(Text),
    /// An enum member.
    Enum(EnumValue),
    /// An option value.
    Option(Rc<OptionValue>),
    /// A record or tuple value.
    Composite(Rc<CompositeValue>),
    /// A collection value.
    Collection(Rc<CollectionValue>),
    /// A terminal object reference.
    Reference(ObjectReference),
}

impl Value {
    /// `occ(v)` of `quire.value.accounting/v1`: one for the value itself plus
    /// every nested occurrence, with `absent` and `null` slots counting zero
    /// and a bag occurrence counted once per multiplicity.
    pub fn occ(&self) -> Integer {
        match self {
            Self::Option(option) => option.occ.clone(),
            Self::Composite(composite) => composite.occ.clone(),
            Self::Collection(collection) => collection.occ().clone(),
            Self::Boolean(_)
            | Self::Integer(_)
            | Self::Rational(_)
            | Self::Decimal(_)
            | Self::Float(_)
            | Self::Quantity(_)
            | Self::Text(_)
            | Self::Enum(_)
            | Self::Reference(_) => Integer::one(),
        }
    }
}

impl fmt::Debug for Value {
    /// Renders the same output `#[derive(Debug)]` would have, but iteratively and in O(n) time: a
    /// depth-first, top-down walk over an explicit worklist writes each byte of the output
    /// exactly once, directly into `f`, at the final indent depth its position in the tree
    /// already determines — so no already-written text is ever re-scanned, re-copied or buffered
    /// as an ancestor's rendering is composed, the way a bottom-up approach that re-embeds each
    /// level's complete rendered text into a fresh string must, and unlike buffering the whole
    /// rendering into one owned `String` before handing it to `f` in a single `write_str` (which
    /// peaks at the size of the entire output).
    ///
    /// The worklist's own peak size is bounded by the total count of not-yet-rendered siblings
    /// across every currently-open ancestor, not by the size of already-written text -- for a
    /// chain (each level has at most one child, as in the stack-depth tests below) that is O(open
    /// ancestors); for one wide collection or record with many elements/fields, `push_list`
    /// (private, below) pushes every element's `Task` onto the worklist before any is popped
    /// and rendered, so the worklist there is O(that level's element count), not O(depth). This
    /// is a characteristic of the eager, all-siblings-at-once expansion this walk has used since it
    /// was made iterative (issue #29), unchanged by IR-55: IR-55 removed the *other* two costs
    /// this impl used to carry (a transient `Vec<Task>` fragment built and copied in per node, and
    /// the whole rendering buffered as one owned `String` before any of it reached `f`), neither
    /// of which is this one. A lazy, one-sibling-at-a-time expansion (an iterator/cursor `Task`
    /// variant rather than pre-pushing a whole slice) would close this remaining gap; not
    /// attempted here.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        render_value(f, self)
    }
}

/// Which bracket pair a block uses. Only affects spacing in compact mode: a struct pads its
/// braces with a space (`Name { f: v }`), a tuple or list does not (`Name(v)`, `[v]`). Alternate
/// mode's indentation is identical for all three.
#[derive(Clone, Copy)]
enum BlockKind {
    Struct,
    TupleOrList,
}

/// Writes one `Value`'s complete `Debug` text straight through to `out` — the caller's
/// `fmt::Formatter`, or any other `fmt::Write` sink — one write per byte, with no owned buffer of
/// its own standing in for any part of the rendering.
struct Writer<'w> {
    out: &'w mut dyn fmt::Write,
    alternate: bool,
}

impl Writer<'_> {
    fn newline_indent(&mut self, depth: usize) -> fmt::Result {
        self.out.write_char('\n')?;
        for _ in 0..depth {
            self.out.write_str("    ")?;
        }
        Ok(())
    }

    /// Written right after an opening bracket/brace/paren, before the first item, at the items'
    /// own `depth` (one deeper than the block's own).
    fn open(&mut self, kind: BlockKind, depth: usize) -> fmt::Result {
        if self.alternate {
            self.newline_indent(depth)
        } else if matches!(kind, BlockKind::Struct) {
            self.out.write_char(' ')
        } else {
            Ok(())
        }
    }

    /// Written between two items, at the items' own `depth`.
    fn between(&mut self, depth: usize) -> fmt::Result {
        if self.alternate {
            self.out.write_char(',')?;
            self.newline_indent(depth)
        } else {
            self.out.write_str(", ")
        }
    }

    /// Written after the last item, before the closing bracket/brace/paren, at the block's own
    /// (one shallower) `depth`.
    fn close(&mut self, kind: BlockKind, depth: usize) -> fmt::Result {
        if self.alternate {
            self.out.write_char(',')?;
            self.newline_indent(depth)
        } else if matches!(kind, BlockKind::Struct) {
            self.out.write_char(' ')
        } else {
            Ok(())
        }
    }

    fn raw(&mut self, text: &str) -> fmt::Result {
        self.out.write_str(text)
    }

    /// Writes one value's own `Debug` rendering directly, as this value's rendering at `depth`,
    /// through `value`'s own `Debug` impl — never pre-rendered to an owned `String` first. In
    /// compact mode this is a plain `write!`. In alternate mode every newline `value`'s own
    /// `Debug` impl emits must be reindented to `depth`, so the write is routed through
    /// [`Reindent`], a `fmt::Write` adaptor that performs that reindenting as each chunk streams
    /// through it, without ever buffering `value`'s rendering as a whole: the cost is proportional
    /// to this one value's own text, never repeated as ancestors compose around it, which is what
    /// keeps the whole walk O(n) instead of the O(n²) a bottom-up re-embedding approach pays (each
    /// ancestor on the path to the root re-scanning and re-copying the same descendant text).
    fn leaf_debug(&mut self, value: &dyn fmt::Debug, depth: usize) -> fmt::Result {
        if self.alternate {
            write!(
                Reindent {
                    inner: &mut *self.out,
                    depth,
                },
                "{value:#?}"
            )
        } else {
            write!(self.out, "{value:?}")
        }
    }
}

/// A `fmt::Write` adaptor that reindents every newline written through it to `depth`, as each
/// chunk arrives, so a value's own (possibly multi-line) `Debug` rendering can be streamed
/// straight to the real output with no intermediate buffer: only the *number* of newlines already
/// written determines the reindenting, which is the same regardless of how the upstream `Debug`
/// impl happens to split its output across `write_str` calls.
struct Reindent<'w> {
    inner: &'w mut dyn fmt::Write,
    depth: usize,
}

impl fmt::Write for Reindent<'_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let mut segments = s.split('\n');
        if let Some(first) = segments.next() {
            self.inner.write_str(first)?;
        }
        for segment in segments {
            self.inner.write_char('\n')?;
            for _ in 0..self.depth {
                self.inner.write_str("    ")?;
            }
            self.inner.write_str(segment)?;
        }
        Ok(())
    }
}

/// One pending write, popped and applied to the [`Writer`] in order. A `Value` (or `FieldValue`,
/// or an `Option<&Value>` payload slot) is expanded into further `Task`s the moment it is popped,
/// never before — so the worklist only ever holds pending work proportional to how many ancestors
/// are still open, not the whole rendered text of any subtree, and each `Value` node is visited
/// exactly once. `Leaf` borrows the leaf's own value rather than owning its rendered text: nothing
/// is rendered until the task is popped, and then it is written straight through, never copied
/// into the worklist first.
enum Task<'a> {
    Value(&'a Value, usize),
    FieldSlot(&'a FieldValue, usize),
    Raw(&'static str),
    Leaf(&'a dyn fmt::Debug, usize),
    Open(BlockKind, usize),
    Between(usize),
    Close(BlockKind, usize),
}

/// Pushes `Name(<inner>)`, single-field tuple-variant shape, at `depth`, directly onto `stack` —
/// no intermediate `Vec<Task>` fragment is built and copied in: `push_inner` pushes the inner
/// content's own tasks straight onto `stack` too, and every push below happens in the reverse of
/// the order the tasks must eventually pop in, so that popping `stack` afterwards replays them
/// name, `(`, inner content (at `depth.saturating_add(1)`), `)`, forward, in order.
fn push_tuple1<'a>(
    stack: &mut Vec<Task<'a>>,
    name: &'static str,
    depth: usize,
    push_inner: impl FnOnce(&mut Vec<Task<'a>>),
) {
    stack.push(Task::Raw(")"));
    stack.push(Task::Close(BlockKind::TupleOrList, depth));
    push_inner(stack);
    stack.push(Task::Open(BlockKind::TupleOrList, depth.saturating_add(1)));
    stack.push(Task::Raw("("));
    stack.push(Task::Raw(name));
}

/// Pushes `Name { f0: v0, f1: v1, f2: v2 }`, exactly the three-field struct shape every `*Value`
/// struct here has, at `depth`, directly onto `stack` (see [`push_tuple1`] for the reverse-push
/// discipline this follows). Each field's push closure receives the depth its own content sits at
/// (`depth.saturating_add(1)`), computed once here rather than re-derived at each call site.
fn push_struct3<'a, F0, F1, F2>(
    stack: &mut Vec<Task<'a>>,
    name: &'static str,
    depth: usize,
    f0: (&'static str, F0),
    f1: (&'static str, F1),
    f2: (&'static str, F2),
) where
    F0: FnOnce(&mut Vec<Task<'a>>, usize),
    F1: FnOnce(&mut Vec<Task<'a>>, usize),
    F2: FnOnce(&mut Vec<Task<'a>>, usize),
{
    let field_depth = depth.saturating_add(1);
    stack.push(Task::Raw("}"));
    stack.push(Task::Close(BlockKind::Struct, depth));
    (f2.1)(stack, field_depth);
    stack.push(Task::Raw(": "));
    stack.push(Task::Raw(f2.0));
    stack.push(Task::Between(field_depth));
    (f1.1)(stack, field_depth);
    stack.push(Task::Raw(": "));
    stack.push(Task::Raw(f1.0));
    stack.push(Task::Between(field_depth));
    (f0.1)(stack, field_depth);
    stack.push(Task::Raw(": "));
    stack.push(Task::Raw(f0.0));
    stack.push(Task::Open(BlockKind::Struct, field_depth));
    stack.push(Task::Raw(" {"));
    stack.push(Task::Raw(name));
}

/// Pushes `[i0, i1, ..]`, at `depth`, directly onto `stack` (see [`push_tuple1`]); each item's own
/// content sits at `depth.saturating_add(1)` and is pushed by `push_item`. `[]` for no items, in
/// both modes — the same "nothing to put on its own line" shape a zero-field struct or tuple
/// would take, had this crate's `Value`-nesting types ever produced one.
fn push_list<'a, T>(
    stack: &mut Vec<Task<'a>>,
    depth: usize,
    items: &'a [T],
    push_item: impl Fn(&mut Vec<Task<'a>>, &'a T, usize),
) {
    if items.is_empty() {
        stack.push(Task::Raw("[]"));
        return;
    }
    let item_depth = depth.saturating_add(1);
    stack.push(Task::Raw("]"));
    stack.push(Task::Close(BlockKind::TupleOrList, depth));
    for (index, item) in items.iter().enumerate().rev() {
        push_item(stack, item, item_depth);
        if index != 0 {
            stack.push(Task::Between(item_depth));
        }
    }
    stack.push(Task::Open(BlockKind::TupleOrList, item_depth));
    stack.push(Task::Raw("["));
}

/// A single non-nesting scalar variant: `Name(<value's own Debug rendering>)`, pushed directly
/// onto `stack`.
fn push_scalar<'a>(
    stack: &mut Vec<Task<'a>>,
    name: &'static str,
    depth: usize,
    value: &'a dyn fmt::Debug,
) {
    push_tuple1(stack, name, depth, |stack| {
        stack.push(Task::Leaf(value, depth.saturating_add(1)));
    });
}

/// Renders `root`'s `Debug` output as a single top-down, depth-first pass over an explicit
/// worklist, writing straight through to `f`: every `Value`, `FieldValue` and `Option` payload
/// slot is matched exhaustively here, with no catch-all arm standing in for "cannot happen" and no
/// fallback text a bookkeeping bug could silently reach for instead of panicking — this crate's
/// governed target treats a stack overflow as silent corruption, so nothing in `src/exact` may
/// carry a panic path, and nothing here needs to reach for one in the first place: every task this
/// walk ever pushes is one this match already knows how to pop.
fn render_value(f: &mut fmt::Formatter<'_>, root: &Value) -> fmt::Result {
    let alternate = f.alternate();
    let mut writer = Writer { out: f, alternate };
    let mut stack: Vec<Task<'_>> = vec![Task::Value(root, 0)];
    while let Some(task) = stack.pop() {
        match task {
            Task::Raw(text) => writer.raw(text)?,
            Task::Leaf(value, depth) => writer.leaf_debug(value, depth)?,
            Task::Open(kind, depth) => writer.open(kind, depth)?,
            Task::Between(depth) => writer.between(depth)?,
            Task::Close(kind, depth) => writer.close(kind, depth)?,

            Task::Value(Value::Boolean(v), depth) => push_scalar(&mut stack, "Boolean", depth, v),
            Task::Value(Value::Integer(v), depth) => push_scalar(&mut stack, "Integer", depth, v),
            Task::Value(Value::Rational(v), depth) => push_scalar(&mut stack, "Rational", depth, v),
            Task::Value(Value::Decimal(v), depth) => push_scalar(&mut stack, "Decimal", depth, v),
            Task::Value(Value::Float(v), depth) => push_scalar(&mut stack, "Float", depth, v),
            Task::Value(Value::Quantity(v), depth) => push_scalar(&mut stack, "Quantity", depth, v),
            Task::Value(Value::Text(v), depth) => push_scalar(&mut stack, "Text", depth, v),
            Task::Value(Value::Enum(v), depth) => push_scalar(&mut stack, "Enum", depth, v),
            Task::Value(Value::Reference(v), depth) => {
                push_scalar(&mut stack, "Reference", depth, v)
            }

            Task::Value(Value::Option(rc), depth) => {
                let option = &**rc;
                push_tuple1(&mut stack, "Option", depth, |stack| {
                    push_struct3(
                        stack,
                        "OptionValue",
                        depth.saturating_add(1),
                        ("payload_type", |stack: &mut Vec<Task<'_>>, fd: usize| {
                            stack.push(Task::Leaf(&option.payload_type, fd));
                        }),
                        (
                            "payload",
                            |stack: &mut Vec<Task<'_>>, fd: usize| match option.payload.as_ref() {
                                None => stack.push(Task::Raw("None")),
                                Some(v) => push_tuple1(stack, "Some", fd, |stack| {
                                    stack.push(Task::Value(v, fd.saturating_add(1)));
                                }),
                            },
                        ),
                        ("occ", |stack: &mut Vec<Task<'_>>, fd: usize| {
                            stack.push(Task::Leaf(&option.occ, fd));
                        }),
                    );
                });
            }
            Task::Value(Value::Composite(rc), depth) => {
                let composite = &**rc;
                push_tuple1(&mut stack, "Composite", depth, |stack| {
                    push_struct3(
                        stack,
                        "CompositeValue",
                        depth.saturating_add(1),
                        ("declaration", |stack: &mut Vec<Task<'_>>, fd: usize| {
                            stack.push(Task::Leaf(&composite.declaration, fd));
                        }),
                        ("slots", |stack: &mut Vec<Task<'_>>, fd: usize| {
                            push_list(stack, fd, &composite.slots, |stack, slot, item_depth| {
                                stack.push(Task::FieldSlot(slot, item_depth));
                            });
                        }),
                        ("occ", |stack: &mut Vec<Task<'_>>, fd: usize| {
                            stack.push(Task::Leaf(&composite.occ, fd));
                        }),
                    );
                });
            }
            Task::Value(Value::Collection(rc), depth) => {
                let collection = &**rc;
                push_tuple1(&mut stack, "Collection", depth, |stack| {
                    push_struct3(
                        stack,
                        "CollectionValue",
                        depth.saturating_add(1),
                        ("collection_type", |stack: &mut Vec<Task<'_>>, fd: usize| {
                            stack.push(Task::Leaf(collection.collection_type(), fd));
                        }),
                        ("elements", |stack: &mut Vec<Task<'_>>, fd: usize| {
                            push_list(
                                stack,
                                fd,
                                collection.elements(),
                                |stack, element, item_depth| {
                                    stack.push(Task::Value(element, item_depth));
                                },
                            );
                        }),
                        ("occ", |stack: &mut Vec<Task<'_>>, fd: usize| {
                            stack.push(Task::Leaf(collection.occ(), fd));
                        }),
                    );
                });
            }

            Task::FieldSlot(FieldValue::Present(v), depth) => {
                push_tuple1(&mut stack, "Present", depth, |stack| {
                    stack.push(Task::Value(v, depth.saturating_add(1)));
                });
            }
            Task::FieldSlot(FieldValue::Absent, _) => writer.raw("Absent")?,
            Task::FieldSlot(FieldValue::Null, _) => writer.raw("Null")?,
        }
    }
    Ok(())
}

impl Drop for Value {
    /// Drains this value's owned nested `Value`s into an explicit worklist rather than letting
    /// the compiler-generated glue recurse with the chain's nesting depth, for the same reason
    /// `Debug` above is iterative: on the governed `thumbv7em-none-eabi` target a stack overflow
    /// is silent memory corruption, not a panic.
    fn drop(&mut self) {
        let mut worklist: Vec<Value> = Vec::new();
        drain_children(self, &mut worklist);
        while let Some(mut next) = worklist.pop() {
            drain_children(&mut next, &mut worklist);
            // `next` drops here. Its own owned children were just drained onto `worklist`, so
            // this nested call back into `Value::drop` finds nothing left to walk: O(1).
        }
    }
}

/// Moves `value`'s directly owned nested `Value`s onto `worklist`, if `value` is the sole owner
/// of its `Rc`. A shared `Rc` (strong count > 1) is left untouched: dropping this `Value` only
/// decrements the refcount, and the contents are still reachable through the other owner.
fn drain_children(value: &mut Value, worklist: &mut Vec<Value>) {
    match value {
        Value::Option(rc) => {
            if let Some(inner) = Rc::get_mut(rc) {
                if let Some(payload) = inner.payload.take() {
                    worklist.push(payload);
                }
            }
        }
        Value::Composite(rc) => {
            if let Some(inner) = Rc::get_mut(rc) {
                for slot in Vec::from(mem::take(&mut inner.slots)) {
                    if let FieldValue::Present(nested) = slot {
                        worklist.push(nested);
                    }
                }
            }
        }
        Value::Collection(rc) => {
            if let Some(inner) = Rc::get_mut(rc) {
                for nested in Vec::from(inner.take_elements()) {
                    worklist.push(nested);
                }
            }
        }
        Value::Boolean(_)
        | Value::Integer(_)
        | Value::Rational(_)
        | Value::Decimal(_)
        | Value::Float(_)
        | Value::Quantity(_)
        | Value::Text(_)
        | Value::Enum(_)
        | Value::Reference(_) => {}
    }
}

/// `none` or a present value of one declared payload type.
#[derive(Clone, Debug)]
pub struct OptionValue {
    payload_type: ValueType,
    payload: Option<Value>,
    occ: Integer,
}

impl OptionValue {
    /// `none` of `Option<payload_type>`.
    pub fn none(payload_type: ValueType) -> Value {
        Value::Option(Rc::new(Self {
            payload_type,
            payload: None,
            occ: Integer::one(),
        }))
    }

    /// A present `payload` of `Option<payload_type>`.
    pub fn present(payload_type: ValueType, payload: Value) -> Result<Value, ConstructionRefusal> {
        if !payload_type.admits(&payload) {
            return Err(ConstructionRefusal {
                component: Component::Payload,
                cause: ConstructionCause::TypeMismatch,
            });
        }
        let occ = Integer::one().add(&payload.occ());
        Ok(Value::Option(Rc::new(Self {
            payload_type,
            payload: Some(payload),
            occ,
        })))
    }

    /// The declared payload type.
    pub fn payload_type(&self) -> &ValueType {
        &self.payload_type
    }

    /// The present payload, or `None` for `none`.
    pub fn payload(&self) -> Option<&Value> {
        self.payload.as_ref()
    }
}

/// Whether a declared field admits only a present value (`f: T`) or also
/// `absent` and explicit `null` (`f: T?`).
///
/// Deliberately not `#[non_exhaustive]` (NFR-002-AC-3, IR-77): a field
/// declaration either carries the grammar's `?` or does not, so this is
/// closed by the grammar rule itself, not by this crate's own evolving
/// vocabulary.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Presence {
    /// A field declared without `?`.
    Required,
    /// A field declared with `?`.
    Optional,
}

/// The state of one field slot.
#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum FieldValue {
    /// A present value.
    Present(Value),
    /// `absent`: the `?` field was omitted.
    Absent,
    /// Explicit `null`, distinct from `absent`.
    Null,
}

/// A declaration-owned named field; its identity is (declaration key, name).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FieldDeclaration {
    name: String,
    value_type: ValueType,
    presence: Presence,
}

impl FieldDeclaration {
    /// The field `name: value_type` or `name: value_type?`.
    pub fn new(name: impl Into<String>, value_type: ValueType, presence: Presence) -> Self {
        Self {
            name: name.into(),
            value_type,
            presence,
        }
    }

    /// The field identifier.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The declared value type.
    pub fn value_type(&self) -> &ValueType {
        &self.value_type
    }

    /// Whether the field was declared with `?`.
    pub fn presence(&self) -> Presence {
        self.presence
    }
}

/// The shape of a composite declaration. Complete V1 has no variant or sum
/// declaration.
#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CompositeShape {
    /// A record with fields in declaration order.
    Record(Vec<FieldDeclaration>),
    /// A tuple of exactly these position types.
    Tuple(Vec<ValueType>),
}

/// A record or tuple declaration with its producer-assigned I04 node key.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompositeDeclaration {
    key: NodeKey,
    name: String,
    shape: CompositeShape,
}

impl CompositeDeclaration {
    /// The declaration `name` with node key `key`.
    pub fn new(key: NodeKey, name: impl Into<String>, shape: CompositeShape) -> Self {
        Self {
            key,
            name: name.into(),
            shape,
        }
    }

    /// The declaration node key.
    pub fn key(&self) -> NodeKey {
        self.key
    }

    /// The declared name, used only to name refusals.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The declared shape.
    pub fn shape(&self) -> &CompositeShape {
        &self.shape
    }
}

/// A model object type exported by a bound model, with its attributes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObjectTypeDeclaration {
    key: NodeKey,
    name: String,
    attributes: Vec<FieldDeclaration>,
}

impl ObjectTypeDeclaration {
    /// The object type `name` with declaration identity `key`.
    pub fn new(key: NodeKey, name: impl Into<String>, attributes: Vec<FieldDeclaration>) -> Self {
        Self {
            key,
            name: name.into(),
            attributes,
        }
    }

    /// The object-type declaration identity.
    pub fn key(&self) -> NodeKey {
        self.key
    }

    /// The declared name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The attributes in declaration order.
    pub fn attributes(&self) -> &[FieldDeclaration] {
        &self.attributes
    }
}

/// Why a declaration set is not admitted.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct InvalidDeclaration {
    /// The name of the declaration where the refusal originates.
    pub declaration: String,
    /// The typed cause.
    pub cause: DeclarationCause,
}

impl fmt::Display for InvalidDeclaration {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "declaration {} refused: {:?}",
            self.declaration, self.cause
        )
    }
}

impl InvalidDeclaration {
    /// The `refused { code }` spelling.
    pub fn code(&self) -> &'static str {
        match self.cause {
            DeclarationCause::DuplicateKey
            | DeclarationCause::DuplicateMember(_)
            | DeclarationCause::UnknownDeclaration(_) => "invalid_semantic_graph",
            DeclarationCause::Type(_) | DeclarationCause::Recursion { .. } => IllTyped::CODE,
        }
    }
}

/// Which recursion-rule subgraph has a cycle.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum RecursionEdges {
    /// A cycle of edges that start at tuple positions.
    Unnamed,
    /// A cycle of edges that pass no `?`, `Option` or minimum-zero collection.
    NonEscaping,
}

/// The typed cause of an [`InvalidDeclaration`].
#[non_exhaustive]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum DeclarationCause {
    /// Two declarations share one node key.
    DuplicateKey,
    /// Two fields or attributes of one declaration share this name.
    DuplicateMember(String),
    /// A type names a key that is no declaration of the package.
    UnknownDeclaration(NodeKey),
    /// A member type is ill-typed: a `Reference<T>` target that is not a model
    /// object type (`type-mismatch`), or an IEEE-bearing set, bag or ordered-set
    /// element type (`operator-ineligible`).
    Type(IllTypedCause),
    /// The recursion rule refuses this cycle of declaration names, whose first
    /// and last entries are the same declaration.
    Recursion {
        /// The offending subgraph.
        edges: RecursionEdges,
        /// The declaration names along the cycle.
        cycle: Vec<String>,
    },
}

/// One checked package's closed, admitted record, tuple and object-type
/// declarations.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TypeEnvironment {
    composites: BTreeMap<NodeKey, CompositeDeclaration>,
    object_types: BTreeMap<NodeKey, ObjectTypeDeclaration>,
}

/// One containment edge of the recursion rule.
#[derive(Clone, Copy)]
struct Edge {
    target: NodeKey,
    named: bool,
    escapes: bool,
}

impl TypeEnvironment {
    /// Admit `composites` and `object_types` as one closed environment.
    pub fn new(
        composites: impl IntoIterator<Item = CompositeDeclaration>,
        object_types: impl IntoIterator<Item = ObjectTypeDeclaration>,
    ) -> Result<Self, InvalidDeclaration> {
        let mut environment = Self::default();
        for declaration in composites {
            let refuse = |cause| InvalidDeclaration {
                declaration: declaration.name.clone(),
                cause,
            };
            if let CompositeShape::Record(fields) = &declaration.shape {
                if let Some(name) = duplicate_name(fields) {
                    return Err(refuse(DeclarationCause::DuplicateMember(name)));
                }
            }
            if environment.object_types.contains_key(&declaration.key)
                || environment.composites.contains_key(&declaration.key)
            {
                return Err(refuse(DeclarationCause::DuplicateKey));
            }
            environment.composites.insert(declaration.key, declaration);
        }
        for declaration in object_types {
            let refuse = |cause| InvalidDeclaration {
                declaration: declaration.name.clone(),
                cause,
            };
            if let Some(name) = duplicate_name(&declaration.attributes) {
                return Err(refuse(DeclarationCause::DuplicateMember(name)));
            }
            if environment.object_types.contains_key(&declaration.key)
                || environment.composites.contains_key(&declaration.key)
            {
                return Err(refuse(DeclarationCause::DuplicateKey));
            }
            environment
                .object_types
                .insert(declaration.key, declaration);
        }
        environment.check_member_types()?;
        environment.check_recursion(RecursionEdges::Unnamed)?;
        environment.check_recursion(RecursionEdges::NonEscaping)?;
        Ok(environment)
    }

    /// The admitted record or tuple declaration with this key.
    pub fn composite(&self, key: NodeKey) -> Option<&CompositeDeclaration> {
        self.composites.get(&key)
    }

    /// Every admitted record and tuple declaration in key order.
    ///
    /// Ported ahead of its caller, the quire-specification/FR-145 expression-machine's static
    /// checker, which is out of scope for this crate; no operator here calls
    /// it yet.
    #[allow(dead_code)]
    pub(crate) fn composites(&self) -> impl Iterator<Item = &CompositeDeclaration> {
        self.composites.values()
    }

    /// The admitted object type with this key.
    pub fn object_type(&self, key: NodeKey) -> Option<&ObjectTypeDeclaration> {
        self.object_types.get(&key)
    }

    /// Check a type named outside a declaration (a parameter or result type):
    /// every named declaration exists, every `Reference<T>` names a model
    /// object type, and no set, bag or ordered set has an IEEE-bearing element
    /// type.
    pub fn check_type(&self, value_type: &ValueType) -> Result<(), IllTyped> {
        match self.type_refusal(value_type) {
            None => Ok(()),
            Some(DeclarationCause::Type(cause)) => Err(IllTyped { cause }),
            Some(_) => Err(IllTyped {
                cause: IllTypedCause::TypeMismatch,
            }),
        }
    }

    /// Whether `value_type` contains `Float32` or `Float64` at any depth,
    /// through record and tuple declarations included.
    pub fn contains_ieee(&self, value_type: &ValueType) -> bool {
        let mut visited = BTreeSet::new();
        let mut pending = vec![value_type];
        while let Some(value_type) = pending.pop() {
            match value_type {
                ValueType::Float(_) => return true,
                ValueType::Option(payload) => pending.push(payload),
                ValueType::Collection(collection) => pending.push(collection.element()),
                ValueType::Composite(key) => {
                    if !visited.insert(*key) {
                        continue;
                    }
                    match self
                        .composites
                        .get(key)
                        .map(|declaration| &declaration.shape)
                    {
                        Some(CompositeShape::Record(fields)) => {
                            pending.extend(fields.iter().map(FieldDeclaration::value_type));
                        }
                        Some(CompositeShape::Tuple(positions)) => pending.extend(positions),
                        None => {}
                    }
                }
                ValueType::Boolean
                | ValueType::Integer
                | ValueType::Int(_)
                | ValueType::Rational(_)
                | ValueType::Decimal(_)
                | ValueType::Quantity(_)
                | ValueType::Text(_)
                | ValueType::Enum(_)
                | ValueType::Reference(_) => {}
            }
        }
        false
    }

    /// The first refusal of one type's named declarations and element types.
    fn type_refusal(&self, value_type: &ValueType) -> Option<DeclarationCause> {
        let mut pending = vec![value_type];
        while let Some(value_type) = pending.pop() {
            match value_type {
                ValueType::Composite(key) if !self.composites.contains_key(key) => {
                    return Some(DeclarationCause::UnknownDeclaration(*key));
                }
                ValueType::Reference(key) if !self.object_types.contains_key(key) => {
                    return Some(if self.composites.contains_key(key) {
                        DeclarationCause::Type(IllTypedCause::TypeMismatch)
                    } else {
                        DeclarationCause::UnknownDeclaration(*key)
                    });
                }
                ValueType::Option(payload) => pending.push(payload),
                ValueType::Collection(collection) => {
                    if collection.kind() != CollectionKind::Sequence
                        && self.contains_ieee(collection.element())
                    {
                        return Some(DeclarationCause::Type(IllTypedCause::OperatorIneligible));
                    }
                    pending.push(collection.element());
                }
                ValueType::Boolean
                | ValueType::Integer
                | ValueType::Int(_)
                | ValueType::Rational(_)
                | ValueType::Decimal(_)
                | ValueType::Float(_)
                | ValueType::Quantity(_)
                | ValueType::Text(_)
                | ValueType::Enum(_)
                | ValueType::Composite(_)
                | ValueType::Reference(_) => {}
            }
        }
        None
    }

    fn check_member_types(&self) -> Result<(), InvalidDeclaration> {
        let composites = self.composites.values().map(|declaration| {
            let types: Vec<&ValueType> = match &declaration.shape {
                CompositeShape::Record(fields) => {
                    fields.iter().map(FieldDeclaration::value_type).collect()
                }
                CompositeShape::Tuple(positions) => positions.iter().collect(),
            };
            (&declaration.name, types)
        });
        let object_types = self.object_types.values().map(|declaration| {
            let types = declaration
                .attributes
                .iter()
                .map(FieldDeclaration::value_type)
                .collect();
            (&declaration.name, types)
        });
        for (name, types) in composites.chain(object_types) {
            if let Some(cause) = types.into_iter().find_map(|ty| self.type_refusal(ty)) {
                return Err(InvalidDeclaration {
                    declaration: name.clone(),
                    cause,
                });
            }
        }
        Ok(())
    }

    /// The recursion-rule edges leaving one declaration.
    fn edges(declaration: &CompositeDeclaration) -> Vec<Edge> {
        let members: Vec<(&ValueType, bool, bool)> = match &declaration.shape {
            CompositeShape::Record(fields) => fields
                .iter()
                .map(|field| {
                    (
                        &field.value_type,
                        true,
                        field.presence == Presence::Optional,
                    )
                })
                .collect(),
            CompositeShape::Tuple(positions) => {
                positions.iter().map(|ty| (ty, false, false)).collect()
            }
        };
        let mut edges = Vec::new();
        for (value_type, named, escapes) in members {
            let mut pending = vec![(value_type, escapes)];
            while let Some((value_type, escapes)) = pending.pop() {
                match value_type {
                    ValueType::Composite(target) => edges.push(Edge {
                        target: *target,
                        named,
                        escapes,
                    }),
                    ValueType::Option(payload) => pending.push((payload, true)),
                    ValueType::Collection(collection) => pending.push((
                        collection.element(),
                        escapes || collection.bound().minimum() == 0,
                    )),
                    ValueType::Boolean
                    | ValueType::Integer
                    | ValueType::Int(_)
                    | ValueType::Rational(_)
                    | ValueType::Decimal(_)
                    | ValueType::Float(_)
                    | ValueType::Quantity(_)
                    | ValueType::Text(_)
                    | ValueType::Enum(_)
                    | ValueType::Reference(_) => {}
                }
            }
        }
        edges
    }

    /// Refuse the first cycle, in declaration-key order, of one recursion-rule
    /// subgraph.
    fn check_recursion(&self, subgraph: RecursionEdges) -> Result<(), InvalidDeclaration> {
        let graph: BTreeMap<NodeKey, Vec<NodeKey>> = self
            .composites
            .values()
            .map(|declaration| {
                let targets = Self::edges(declaration)
                    .into_iter()
                    .filter(|edge| match subgraph {
                        RecursionEdges::Unnamed => !edge.named,
                        RecursionEdges::NonEscaping => !edge.escapes,
                    })
                    .map(|edge| edge.target)
                    .collect();
                (declaration.key, targets)
            })
            .collect();
        let name = |key: &NodeKey| {
            self.composites
                .get(key)
                .map_or_else(String::new, |declaration| declaration.name.clone())
        };
        let mut finished = BTreeSet::new();
        for root in graph.keys() {
            if finished.contains(root) {
                continue;
            }
            let mut path: Vec<(NodeKey, usize)> = vec![(*root, 0)];
            while let Some((node, next)) = path.last_mut() {
                let node = *node;
                let Some(target) = graph.get(&node).and_then(|targets| targets.get(*next)) else {
                    finished.insert(node);
                    path.pop();
                    continue;
                };
                *next = next.saturating_add(1);
                if let Some(start) = path.iter().position(|(on_path, _)| on_path == target) {
                    let mut cycle: Vec<String> =
                        path.iter().skip(start).map(|(key, _)| name(key)).collect();
                    cycle.push(name(target));
                    return Err(InvalidDeclaration {
                        declaration: name(target),
                        cause: DeclarationCause::Recursion {
                            edges: subgraph,
                            cycle,
                        },
                    });
                }
                if !finished.contains(target) {
                    path.push((*target, 0));
                }
            }
        }
        Ok(())
    }

    /// Construct a record from its supplied fields. An omitted `?` field is
    /// `absent`.
    pub fn record(
        &self,
        declaration: NodeKey,
        fields: Vec<(&str, FieldValue)>,
    ) -> Result<Value, ConstructionRefusal> {
        let Some(CompositeShape::Record(declared)) = self.shape(declaration) else {
            return refuse(Component::Value, ConstructionCause::UnknownDeclaration);
        };
        let slots = fill_slots(declared, fields)?;
        Ok(composite(declaration, slots))
    }

    /// Construct a tuple of exactly its declared arity.
    pub fn tuple(
        &self,
        declaration: NodeKey,
        positions: Vec<Value>,
    ) -> Result<Value, ConstructionRefusal> {
        let declared = self.tuple_positions(declaration, positions.len())?;
        if let Some(position) = declared
            .iter()
            .zip(&positions)
            .position(|(value_type, value)| !value_type.admits(value))
        {
            return refuse(
                Component::Position(position),
                ConstructionCause::TypeMismatch,
            );
        }
        let slots = positions.into_iter().map(FieldValue::Present).collect();
        Ok(composite(declaration, slots))
    }

    /// Evaluate a record value expression. Every construction refusal is
    /// decided before any field expression runs. Field expressions then run in
    /// declaration order, whatever the source order; the first one that does
    /// not complete becomes the outcome and no later one runs. A completed
    /// record charges `composite.result-retain`.
    pub fn evaluate_record(
        &self,
        declaration: NodeKey,
        fields: Vec<(&str, FieldExpression<'_>)>,
        meter: &mut Meter,
    ) -> Result<Outcome<Value>, ConstructionRefusal> {
        let Some(CompositeShape::Record(declared)) = self.shape(declaration) else {
            return refuse(Component::Value, ConstructionCause::UnknownDeclaration);
        };
        let mut supplied = match_names(declared, fields)?;
        let mut plan = Vec::with_capacity(declared.len());
        for field in declared {
            let expression = supplied.remove(field.name.as_str());
            let component = || Component::Field(field.name.clone());
            match (&expression, field.presence) {
                (None, Presence::Required) => {
                    return refuse(component(), ConstructionCause::MissingField)
                }
                (Some(FieldExpression::Null), Presence::Required) => {
                    return refuse(component(), ConstructionCause::NullForRequiredField)
                }
                (None | Some(FieldExpression::Null | FieldExpression::Evaluate(_)), _) => {}
            }
            plan.push((field, expression));
        }
        let mut slots = Vec::with_capacity(plan.len());
        for (field, expression) in plan {
            let slot = match expression {
                None => FieldValue::Absent,
                Some(FieldExpression::Null) => FieldValue::Null,
                Some(FieldExpression::Evaluate(expression)) => {
                    match admitted(&field.value_type, expression(meter)) {
                        Ok(value) => FieldValue::Present(value),
                        Err(stop) => return Ok(Outcome::from_stop(Err(stop))),
                    }
                }
            };
            slots.push(slot);
        }
        Ok(Outcome::from_stop(retain_composite(
            composite(declaration, slots.into_boxed_slice()),
            meter,
        )))
    }

    /// Evaluate a tuple call `T(e, ...)`: the arity is checked first, then the
    /// arguments run in position order under the first-stopped rule, then a
    /// completed tuple charges `composite.result-retain`.
    pub fn evaluate_tuple(
        &self,
        declaration: NodeKey,
        positions: Vec<Deferred<'_>>,
        meter: &mut Meter,
    ) -> Result<Outcome<Value>, ConstructionRefusal> {
        let declared = self.tuple_positions(declaration, positions.len())?;
        let mut slots = Vec::with_capacity(declared.len());
        for (value_type, expression) in declared.iter().zip(positions) {
            match admitted(value_type, expression(meter)) {
                Ok(value) => slots.push(FieldValue::Present(value)),
                Err(stop) => return Ok(Outcome::from_stop(Err(stop))),
            }
        }
        Ok(Outcome::from_stop(retain_composite(
            composite(declaration, slots.into_boxed_slice()),
            meter,
        )))
    }

    fn tuple_positions(
        &self,
        declaration: NodeKey,
        supplied: usize,
    ) -> Result<&[ValueType], ConstructionRefusal> {
        let Some(CompositeShape::Tuple(declared)) = self.shape(declaration) else {
            return refuse(Component::Value, ConstructionCause::UnknownDeclaration);
        };
        if declared.len() != supplied {
            return refuse(
                Component::Value,
                ConstructionCause::WrongArity {
                    declared: declared.len(),
                    supplied,
                },
            );
        }
        Ok(declared)
    }

    fn shape(&self, declaration: NodeKey) -> Option<&CompositeShape> {
        self.composites.get(&declaration).map(|d| &d.shape)
    }
}

/// A deferred expression: it runs only when construction reaches it.
pub type Deferred<'a> = Box<dyn FnOnce(&mut Meter) -> Outcome<Value> + 'a>;

/// A record field in a record value expression. Omitting a `?` field
/// constructs `absent`.
#[non_exhaustive]
pub enum FieldExpression<'a> {
    /// `f: e`.
    Evaluate(Deferred<'a>),
    /// `f: null`.
    Null,
}

impl fmt::Debug for FieldExpression<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Evaluate(_) => formatter.write_str("Evaluate(..)"),
            Self::Null => formatter.write_str("Null"),
        }
    }
}

/// The completed value of a deferred expression, which a checked program
/// guarantees is a member of `value_type`.
fn admitted(value_type: &ValueType, outcome: Outcome<Value>) -> Result<Value, Stop> {
    let value = outcome.into_stop()?;
    if value_type.admits(&value) {
        Ok(value)
    } else {
        Err(Stop::Refused(Refusal::CheckedInvariant))
    }
}

/// Charge `composite.result-retain` with `occ(result)`, then expose it.
pub(crate) fn retain_composite(value: Value, meter: &mut Meter) -> Result<Value, Stop> {
    let occ = value.occ();
    meter.charge(
        Charge::new(ChargePoint::CompositeResultRetain)
            .exact_size(LimitKind::ValueOccurrences, occ.clone())
            .exact_results(occ),
    )?;
    Ok(value)
}

/// Where a construction refusal originates.
#[non_exhaustive]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Component {
    /// The composite value as a whole (its declaration or arity).
    Value,
    /// A named record field or object attribute.
    Field(String),
    /// A zero-based tuple position.
    Position(usize),
    /// A zero-based collection occurrence in source order.
    Element(usize),
    /// An option payload.
    Payload,
}

/// Why a construction is `refused { code: ill_typed }`.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ConstructionCause {
    /// The declaration is not a record or tuple of the environment, or has
    /// the other shape.
    UnknownDeclaration,
    /// A required field is omitted.
    MissingField,
    /// A supplied field is not declared.
    UndeclaredField,
    /// A field is supplied twice.
    DuplicateField,
    /// `null` is supplied for a required field.
    NullForRequiredField,
    /// A tuple call has another argument count than its declared arity.
    WrongArity {
        /// Declared arity.
        declared: usize,
        /// Supplied arguments.
        supplied: usize,
    },
    /// A value that is not a member of the declared type.
    TypeMismatch,
}

/// A typed construction refusal at its originating component.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ConstructionRefusal {
    /// The originating component.
    pub component: Component,
    /// The typed cause.
    pub cause: ConstructionCause,
}

impl fmt::Display for ConstructionRefusal {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "construction refused at {:?}: {:?}",
            self.component, self.cause
        )
    }
}

impl ConstructionRefusal {
    /// The `refused { code }` spelling.
    pub const CODE: &'static str = IllTyped::CODE;
}

fn refuse<T>(component: Component, cause: ConstructionCause) -> Result<T, ConstructionRefusal> {
    Err(ConstructionRefusal { component, cause })
}

fn duplicate_name(fields: &[FieldDeclaration]) -> Option<String> {
    let mut seen = BTreeSet::new();
    fields
        .iter()
        .find(|field| !seen.insert(field.name.as_str()))
        .map(|field| field.name.clone())
}

/// Index supplied entries by declared name, refusing an undeclared or
/// repeated name.
fn match_names<'n, T>(
    declared: &[FieldDeclaration],
    supplied: Vec<(&'n str, T)>,
) -> Result<BTreeMap<&'n str, T>, ConstructionRefusal> {
    let mut by_name = BTreeMap::new();
    for (name, entry) in supplied {
        let component = || Component::Field(name.into());
        if !declared.iter().any(|field| field.name == name) {
            return refuse(component(), ConstructionCause::UndeclaredField);
        }
        if by_name.insert(name, entry).is_some() {
            return refuse(component(), ConstructionCause::DuplicateField);
        }
    }
    Ok(by_name)
}

/// Declaration-ordered slots of a record or object from supplied fields.
pub(crate) fn fill_slots(
    declared: &[FieldDeclaration],
    supplied: Vec<(&str, FieldValue)>,
) -> Result<Box<[FieldValue]>, ConstructionRefusal> {
    let mut by_name = match_names(declared, supplied)?;
    let mut slots = Vec::with_capacity(declared.len());
    for field in declared {
        let component = || Component::Field(field.name.clone());
        let slot = by_name
            .remove(field.name.as_str())
            .unwrap_or(FieldValue::Absent);
        match (&slot, field.presence) {
            (FieldValue::Absent, Presence::Required) => {
                return refuse(component(), ConstructionCause::MissingField)
            }
            (FieldValue::Null, Presence::Required) => {
                return refuse(component(), ConstructionCause::NullForRequiredField)
            }
            (FieldValue::Present(value), _) if !field.value_type.admits(value) => {
                return refuse(component(), ConstructionCause::TypeMismatch)
            }
            (FieldValue::Present(_) | FieldValue::Absent | FieldValue::Null, _) => {}
        }
        slots.push(slot);
    }
    Ok(slots.into_boxed_slice())
}

/// `1 + occ` of every present slot.
pub(crate) fn slots_occ(slots: &[FieldValue]) -> Integer {
    slots.iter().fold(Integer::one(), |occ, slot| match slot {
        FieldValue::Present(value) => occ.add(&value.occ()),
        FieldValue::Absent | FieldValue::Null => occ,
    })
}

fn composite(declaration: NodeKey, slots: Box<[FieldValue]>) -> Value {
    let occ = slots_occ(&slots);
    Value::Composite(Rc::new(CompositeValue {
        declaration,
        slots,
        occ,
    }))
}

/// A record or tuple value.
#[derive(Clone, Debug)]
pub struct CompositeValue {
    declaration: NodeKey,
    slots: Box<[FieldValue]>,
    occ: Integer,
}

impl CompositeValue {
    /// The declaration node key.
    pub fn declaration(&self) -> NodeKey {
        self.declaration
    }

    /// Field or position states in declaration order.
    pub fn slots(&self) -> &[FieldValue] {
        &self.slots
    }
}
