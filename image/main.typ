#{
  /// ====== Config ======
  let SERIF-FONTS = ("STIX Two Text", "Source Han Serif SC")
  let MONO-FONTS = ("Fira Code Retina", "PingFang SC")
  
  let correct-color = rgb("#1d9c9c")
  let misplaced-color = rgb("#de7525")
  let bg-color = rgb("#f7f8fa")
  let fg-color = rgb("#5f6672")
  let missing-color = rgb("#b5b8be")
  
  let box-size = 5em
  
  /// ====== Styles ======
  
  set text(font: SERIF-FONTS)
  
  set page(
    height: auto,
    width: auto,
    margin: 1em,
  )
  
  show grid.cell: it => box(
    height: box-size,
    width: box-size,
    fill: rgb("#f7f8fa"),
    it,
  )
  
  /// ====== Models ======
  
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
  
  let json-path = sys.inputs.at("path", default: "./data.json")
  let data = json(json-path)
  
  // repr(data)
  
  let underlined(color, it) = if color == none {
    it
  } else {
    underline(
      stroke: (
        paint: color,
        thickness: .05em,
      ),
      extent: .15em / 2,
      offset: .15em,
      it,
    )
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
    }
    
    box(
      width: box-size,
      height: box-size,
      fill: whole-color,
      align(
        center + horizon,
        pad(
          bottom: .25em,
          stack(
            dir: ttb,
            spacing: .75em,
            stack(
              dir: ltr,
              {
                set text(
                  size: .9em,
                  font: MONO-FONTS,
                )
                
                h(.5em)
                underlined(
                  underline-color,
                  {
                    text(fill: initial-color, initial)
                    h(.15em)
                    text(fill: vowel-color, vowel)
                  },
                )
                h(.1em)
                let tone-str = tone-to-str(tone)
                if tone-str == none {
                  h(.5em)
                } else {
                  text(size: .75em, baseline: -.5em, fill: tone-color, tone-to-str(tone))
                }
              },
            ),
            text(2em, fill: literal-color, literal),
          ),
        ),
      ),
    )
  }
  
  let rows = data.result.map(it => {
    it.map(it => make-cell(
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
  })
  
  // pagebreak()
  
  /// ====== Content ======
  
  grid(
    columns: 4,
    rows: calc.min(
      data.max_attempt_count,
      data.result.len() + if data.finished {
        0
      } else {
        1
      },
    ),
    column-gutter: .5em,
    row-gutter: .5em,
    ..rows.flatten(),
  )
  
  align(center)[#data.result.len() \/ #data.max_attempt_count]
}
