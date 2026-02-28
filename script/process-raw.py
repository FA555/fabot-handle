# -*- coding: utf-8 -*-


import json

'''
raw data:
[
    {
        "derivation": "语出《法华经·法师功德品》下至阿鼻地狱。”",
        "example": "但也有少数意志薄弱的……逐步上当，终至堕入～。★《上饶集中营·炼狱杂记》",
        "explanation": "阿鼻梵语的译音，意译为无间”，即痛苦无有间断之意。常用来比喻黑暗的社会和严酷的牢狱。又比喻无法摆脱的极其痛苦的境地。",
        "pinyin": "ā bí dì yù",
        "word": "阿鼻地狱",
        "abbreviation": "abdy"
    },
    {
        "derivation": "三国·魏·曹操《整齐风俗令》阿党比周，先圣所疾也。”",
        "example": "《论语·卫灵公》众恶之，必察焉；众好之，必察焉”何晏集解引三国魏王肃曰或众～，或其人特立不群，故好恶不可不察也。”",
        "explanation": "指相互勾结，相互偏袒，结党营私。",
        "pinyin": "ē dǎng bǐ zhōu",
        "word": "阿党比周",
        "abbreviation": "edbz"
    },
    ...
]

expected output:
[
    {
        "explanation": "阿鼻梵语的译音，意译为无间”，即痛苦无有间断之意。常用来比喻黑暗的社会和严酷的牢狱。又比喻无法摆脱的极其痛苦的境地。",
        "pinyin": "a1 bi2 di4 yu4",
        "word": "阿鼻地狱",
    },
    {
        "explanation": "指相互勾结，相互偏袒，结党营私。",
        "pinyin": "e1 dang3 bi3 zhou1",
        "word": "阿党比周",
    },
    ...
]
'''

PINYIN_MAP = {
    'ā': ('a', 1),
    'á': ('a', 2),
    'ǎ': ('a', 3),
    'à': ('a', 4),
    'ō': ('o', 1),
    'ó': ('o', 2),
    'ǒ': ('o', 3),
    'ò': ('o', 4),
    'ē': ('e', 1),
    'é': ('e', 2),
    'ě': ('e', 3),
    'è': ('e', 4),
    'ī': ('i', 1),
    'í': ('i', 2),
    'ǐ': ('i', 3),
    'ì': ('i', 4),
    'ū': ('u', 1),
    'ú': ('u', 2),
    'ǔ': ('u', 3),
    'ù': ('u', 4),
    'ü': ('ü', 0),
    'ǘ': ('ü', 2),
    'ǚ': ('ü', 3),
    'ǜ': ('ü', 4),
}


def standardize_pinyin(pinyin):
    for k, v in PINYIN_MAP.items():
        if k in pinyin:
            pinyin = pinyin.replace(k, v[0]) + str(v[1])
            break
    return pinyin


def standardize_pinyins(pinyins):
    return ' '.join(map(standardize_pinyin, pinyins.split(' ')))


def process_raw_data(raw_data):
    def mapper(item):
        return {
            'word': item['word'],
            'pinyin': standardize_pinyins(item['pinyin']),
            'explanation': item['explanation']
        }

    return list(map(mapper, filter(lambda item: len(item['word']) == 4, raw_data)))


def main():
    with open('data/raw/all.json', 'r') as f:
        raw_data = json.load(f)

    with open('data/all.json', 'w') as f:
        json.dump(process_raw_data(raw_data), f, indent=4, ensure_ascii=False)


if __name__ == '__main__':
    main()
