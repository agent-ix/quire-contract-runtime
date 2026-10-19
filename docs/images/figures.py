# SPDX-License-Identifier: AGPL-3.0-or-later
# Copyright (C) 2026 Agent-IX
"""quire-contract-runtime's documentation figures, rendered by the docs-figures skill.

Regenerate from the repository root with the agent-ix/dev-tools skill:

    python3 -m pip install fonttools
    python3 <dev-tools>/skills/docs-figures/scripts/render.py docs/images/figures.py

Add --check to confirm the committed SVGs are current.
"""

from docs_figures import FONTS, SMALL, TITLE, Figure  # noqa: F401

STYLE = "ix-docs"

# One role per kind of thing, fixed across the Quire repositories.
ROLES = [
    ("code", "authored source", "box"),
    ("logic", "compiler stage", "box"),
    ("model", "checked package / IR", "box"),
    ("evidence", "outcome", "box"),
    ("ink", "input", "box"),
]


def hero():
    """README hero: a generated oracle calls the runtime and gets one of three verdicts."""
    f = Figure(
        "hero",
        980,
        350,
        "A generated oracle passes observed values to quire-contract-runtime, which returns one "
        "of three verdicts: Passed, FailedPostcondition or RejectedPrecondition. A campaign report "
        "counts every verdict.",
    )
    f.box(0, 100, 160, 56, "ink", "observed values", "inputs and results")
    f.box(210, 100, 220, 56, "model", "generated oracle", "from quire-contract-codegen")
    f.box(480, 100, 190, 56, "logic", "quire-contract-runtime", "checked ops · clauses")
    f.line([(160, 128), (207, 128)], "ink")
    f.line([(430, 128), (477, 128)], "model")
    trunk = 698
    f.line([(670, 128), (trunk, 128)], "logic", arrow=False)
    f.line([(trunk, 28), (trunk, 180)], "logic", arrow=False)
    verdicts = [
        (0, "Passed", "all required clauses held"),
        (76, "FailedPostcondition", "accepted, postcondition false"),
        (152, "RejectedPrecondition", "case outside the input domain"),
    ]
    for y, name, sub in verdicts:
        f.line([(trunk, y + 28), (717, y + 28)], "logic")
        f.box(720, y, 260, 56, "evidence", name, sub)
    f.line([(850, 208), (850, 257)], "evidence")
    f.note(838, 238, "each verdict is recorded", "muted", anchor="end")
    f.box(720, 260, 260, 56, "model", "CampaignReport", "four saturating counts")
    f.text(480, 190, "no conversion to bool", "muted", "mono", SMALL)
    f.legend(0, 334, [ROLES[0], ROLES[1], ROLES[2], ("evidence", "verdict", "box"), ROLES[4]])
    return f


REPOS = [
    ("ORCHESTRATION", [("quire-verification", True), ("quire-driver", True), ("quire-integration", True)]),
    ("BACKENDS", [("quire-contract-codegen", False), ("quire-contract-runtime", False),
                  ("quire-analyze", False), ("quire-protocol", True), ("quire-mltl", False)]),
    ("LANGUAGE", [("quire-specification", True), ("quire-spec-language", False), ("quire-contract-ir", False)]),
    ("FOUNDATIONS", [("quire-exact", False), ("quire-semantic-value", False), ("quire-walk", False),
                     ("quire-canonical", False), ("quire-verification-contracts", False),
                     ("quire-observation", False)]),
]


def ecosystem(here="quire-spec-language"):
    """The Quire repositories in bands; each band builds on the ones below."""
    f = Figure(
        "ecosystem",
        980,
        364,
        "The Quire repositories in four bands. Foundations: quire-exact, quire-semantic-value, "
        "quire-walk, quire-canonical, quire-verification-contracts, quire-observation. Language: "
        "quire-specification, quire-spec-language, quire-contract-ir. Backends: "
        "quire-contract-codegen, quire-contract-runtime, quire-analyze, quire-protocol, "
        "quire-mltl. Orchestration: quire-verification, quire-driver, quire-integration.",
    )
    y = 0
    gap = 10
    for band, repos in REPOS:
        f.text(0, y + 12, band, "muted", "mono", SMALL)
        widths = [FONTS["title"].width(name, TITLE) + 24 for name, _ in repos]
        spare = (980 - sum(widths) - gap * (len(repos) - 1)) / len(repos)
        x = 0
        for (name, private), w in zip(repos, widths):
            w = round(w + spare)
            role = "logic" if name == here else "ink"
            f.box(x, y + 20, w, 50, role, name, "private" if private else "public", dashed=private)
            x += w + gap
        y += 88
    f.legend(0, 360, [("logic", "this repository", "box"), ("ink", "private", "dash")])
    return f


def ecosystem_here():
    """This repository's highlighted copy of the ecosystem figure."""
    return ecosystem(here="quire-contract-runtime")


FIGURES = [hero, ecosystem_here]
