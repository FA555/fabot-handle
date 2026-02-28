#{
  /// ====== Config ======
  let fonts = (
    serif: ("Libertinus Serif", "LXGW Neo ZhiSong"),
    mono: "Fira Code Retina",
  )

  let correct-color = rgb("#1d9c9c")
  let misplaced-color = rgb("#de7525")
  let bg-color = rgb("#f7f8fa")
  let fg-color = rgb("#5f6672")
  let missing-color = rgb("#b5b8be")

  let cell-size = 5em

  /// ====== Styles ======

  set text(font: fonts.serif)

  set page(
    height: auto,
    width: auto,
    margin: (left: 1em, right: .5em, top: 1.5em, bottom: .75em),
    header: align(right, text(size: .65em, font: fonts.mono, fill: gray)[\@fa_555 Handle Bot]),
  )

  /// ====== Models ======

  let check = () => {
    set curve(stroke: (
      thickness: .125em,
      paint: correct-color,
      cap: "round",
    ))

    show: box.with(width: 1em, height: 1em)
    show: place.with(center + horizon, dy: .7em)
    stack(
      dir: ltr,
      curve(curve.line((-.25em, -.25em))),
      curve(curve.line((.5em, -.5em))),
    )
  }

  let State = (
    missing: "Missing",
    misplaced: "Misplaced",
    correct: "Correct",
  )

  let reverse-state-map = (
    "Missing": State.missing,
    "Misplaced": State.misplaced,
    "Correct": State.correct,
  )

  let tone-to-str = tone => if tone == none {
    none
  } else {
    (
      "High": "1",
      "Rising": "2",
      "Low": "3",
      "Falling": "4",
    ).at(tone)
  }

  /// ====== Data ======

  let json-path = sys.inputs.at("path", default: "./mock-data.json")
  let data = json(json-path)

  // repr(data)

  let underlined(color, it) = if color == none {
    it
  } else {
    underline(stroke: (paint: color, thickness: .05em), extent: .15em / 2, offset: .15em, it)
  }

  let make-cell(
    literal,
    initial,
    vowel,
    tone,
    whole-color: none,
    literal-color: none,
    underline-color: none,
    initial-color: none,
    vowel-color: none,
    tone-color: none,
  ) = {
    if whole-color != none {
      literal-color = white
      underline-color = none
      initial-color = white
      vowel-color = white
      tone-color = white
    } else {
      whole-color = bg-color
    }

    show: grid.cell
    show: box.with(width: cell-size, height: cell-size, fill: whole-color)
    show: align.with(center + horizon)
    show: pad.with(bottom: .25em)
    stack(
      dir: ttb,
      spacing: .75em,
      {
        set text(size: .9em, font: fonts.mono)

        // 左侧 spacing 用于视觉平衡
        h(.5em)

        // 声母 + 韵母
        underlined(
          underline-color,
          {
            text(fill: initial-color, initial)
            h(.15em)
            text(fill: vowel-color, vowel)
          },
        )

        h(.1em)

        // 声调
        let tone-str = tone-to-str(tone)
        if tone-str == none {
          h(.5em)
        } else {
          text(size: .75em, baseline: -.5em, fill: tone-color, tone-str)
        }
      },
      text(2em, fill: literal-color, literal),
    )
  }

  let rows = data.result.map(it => {
    it.characters.map(it => make-cell(
      it.literal,
      it.pinyin.initial,
      it.pinyin.vowel,
      it.pinyin.tone,
      whole-color: if it.result.whole == "Correct" {
        correct-color
      } else {
        none
      },
      underline-color: if it.result.pronunciation == "Correct" {
        correct-color
      } else if it.result.pronunciation == "Misplaced" {
        misplaced-color
      },
      literal-color: if it.result.whole == "Misplaced" {
        misplaced-color
      } else {
        fg-color
      },
      initial-color: if it.result.initial == "Correct" {
        correct-color
      } else if it.result.initial == "Misplaced" {
        misplaced-color
      } else {
        missing-color
      },
      vowel-color: if it.result.vowel == "Correct" {
        correct-color
      } else if it.result.vowel == "Misplaced" {
        misplaced-color
      } else {
        missing-color
      },
      tone-color: if it.result.tone == "Correct" {
        correct-color
      } else if it.result.tone == "Misplaced" {
        misplaced-color
      } else {
        missing-color
      },
    ))

    (
      if it.verified {
        align(
          center + horizon,
          check(),
        )
      },
    )
  })

  /// ====== Content ======

  grid(
    columns: 5,
    rows: calc.min(
      data.max_attempt_count,
      data.result.len() + if data.finished { 0 } else { 1 },
    ),
    gutter: .5em,

    ..rows.flatten(),

    // 如果还没结束，给一行空行
    ..if not data.finished {
      (
        (
          box(
            width: cell-size,
            height: cell-size,
            fill: bg-color,
          ),
        )
          * 4
      )
    },
  )

  align(center, {
    str(data.result.len())
    " / "
    str(data.max_attempt_count)
  })
}
