

### 1. `anime_coloring.py` (Python)

```python
# anime_coloring.py — Python версия

import sys
import argparse

# Карта цветов ANSI
COLORS = {
    'black': 30, 'red': 31, 'green': 32, 'yellow': 33,
    'blue': 34, 'magenta': 35, 'cyan': 36, 'white': 37,
    'gray': 90, 'bright_red': 91, 'bright_green': 92,
    'bright_yellow': 93, 'bright_blue': 94, 'bright_magenta': 95,
    'bright_cyan': 96, 'bright_white': 97
}

# ASCII-арт персонажа с областями
# Формат: [метка] для замены цветом
TEMPLATE = """
          {hair}██{hair}██{hair}██{hair}██{hair}██
        {hair}██{skin}      {skin}{hair}██
       {hair}██  {eye}██{hair}  {hair}██  {eye}██{hair}  {hair}██
       {hair}██  {eye}██{hair}  {hair}██  {eye}██{hair}  {hair}██
        {hair}██{skin}      {skin}{hair}██
         {hair}██{hair}██{hair}██{hair}██{hair}██{hair}██{hair}██{hair}
          {hair}██  {hair}██
         {hair}██{clothes}    {clothes}{hair}██
         {hair}██{clothes}    {clothes}{hair}██
          {hair}██{hair}██{hair}██{hair}██{hair}██
"""

def colorize(text, color_name):
    """Обёртывает текст в ANSI-код цвета."""
    if color_name not in COLORS:
        color_name = 'white'
    code = COLORS[color_name]
    return f"\033[{code}m{text}\033[0m"

def generate_image(colors):
    """Заменяет метки в шаблоне на цветные символы."""
    result = TEMPLATE
    for key, value in colors.items():
        # Заменяем {key} на цветной текст с символом '█' (или пробелом для skin)
        if key == 'skin':
            replacement = colorize('  ', value)  # два пробела для кожи
        else:
            replacement = colorize('██', value)
        result = result.replace('{' + key + '}', replacement)
    # Убираем лишние пробелы, оставляя структуру
    lines = result.split('\n')
    # Обрезаем пробелы в начале каждой строки (сохраняя отступы)
    return '\n'.join(lines)

def main():
    parser = argparse.ArgumentParser(description='Anime Coloring')
    parser.add_argument('--hair', default='magenta', help='Цвет волос')
    parser.add_argument('--eyes', default='cyan', help='Цвет глаз')
    parser.add_argument('--skin', default='yellow', help='Цвет кожи')
    parser.add_argument('--clothes', default='blue', help='Цвет одежды')
    parser.add_argument('--output', '-o', help='Сохранить в файл')
    args = parser.parse_args()

    colors = {
        'hair': args.hair,
        'eyes': args.eyes,
        'skin': args.skin,
        'clothes': args.clothes
    }

    image = generate_image(colors)
    print("🎨 Anime Coloring (Python)")
    print("Цвета: волосы={}, глаза={}, кожа={}, одежда={}".format(
        args.hair, args.eyes, args.skin, args.clothes))
    print()
    print(image)

    if args.output:
        # Сохраняем с ANSI-кодами
        with open(args.output, 'w') as f:
            f.write(image + '\n')
        print(f"💾 Сохранено в {args.output}")

if __name__ == '__main__':
    main()
