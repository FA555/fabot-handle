# -*- coding: utf-8 -*-


import json


def main():
    with open('data/all.json', 'r') as f:
        all_idiom_items = json.load(f)
    with open('data/raw/answer.txt', 'r') as f:
        answer_lines = f.read().splitlines()

    reverse_index = {}
    for i, idiom in enumerate(all_idiom_items):
        reverse_index[idiom['word']] = i

    processed_idioms = []
    for line in answer_lines:
        idiom = line.split()[0]
        if idiom in reverse_index:
            processed_idioms.append(all_idiom_items[reverse_index[idiom]])
        if len(processed_idioms) == 3000:
            break
    
    with open('data/high-frequency.json', 'w') as f:
        json.dump(processed_idioms, f, ensure_ascii=False, indent=4)


if __name__ == '__main__':
    main()
