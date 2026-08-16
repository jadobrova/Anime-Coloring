// anime_coloring.js — JavaScript версия

const fs = require('fs');

const COLORS = {
    black: 30, red: 31, green: 32, yellow: 33,
    blue: 34, magenta: 35, cyan: 36, white: 37,
    gray: 90, bright_red: 91, bright_green: 92,
    bright_yellow: 93, bright_blue: 94, bright_magenta: 95,
    bright_cyan: 96, bright_white: 97
};

const TEMPLATE = `
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
`;

function colorize(text, colorName) {
    const code = COLORS[colorName] || 37;
    return `\x1b[${code}m${text}\x1b[0m`;
}

function generateImage(hair, eyes, skin, clothes) {
    let result = TEMPLATE;
    result = result.replace(/\{hair\}/g, colorize('██', hair));
    result = result.replace(/\{eyes\}/g, colorize('██', eyes));
    result = result.replace(/\{skin\}/g, colorize('  ', skin));
    result = result.replace(/\{clothes\}/g, colorize('██', clothes));
    return result;
}

function main() {
    const args = process.argv.slice(2);
    let hair = 'magenta', eyes = 'cyan', skin = 'yellow', clothes = 'blue';
    let output = null;

    for (let i = 0; i < args.length; i++) {
        if (args[i] === '--hair') hair = args[++i];
        else if (args[i] === '--eyes') eyes = args[++i];
        else if (args[i] === '--skin') skin = args[++i];
        else if (args[i] === '--clothes') clothes = args[++i];
        else if (args[i] === '--output') output = args[++i];
    }

    console.log('🎨 Anime Coloring (JavaScript)');
    console.log(`Цвета: волосы=${hair}, глаза=${eyes}, кожа=${skin}, одежда=${clothes}`);
    console.log();
    const image = generateImage(hair, eyes, skin, clothes);
    console.log(image);

    if (output) {
        fs.writeFileSync(output, image + '\n');
        console.log(`💾 Сохранено в ${output}`);
    }
}

if (require.main === module) main();
