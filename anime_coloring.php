<?php
// anime_coloring.php — PHP версия

$COLORS = [
    'black' => 30, 'red' => 31, 'green' => 32, 'yellow' => 33,
    'blue' => 34, 'magenta' => 35, 'cyan' => 36, 'white' => 37,
    'gray' => 90, 'bright_red' => 91, 'bright_green' => 92,
    'bright_yellow' => 93, 'bright_blue' => 94, 'bright_magenta' => 95,
    'bright_cyan' => 96, 'bright_white' => 97
];

$TEMPLATE = "
          {hair}██{hair}██{hair}██{hair}██{hair}██
        {hair}██{skin}      {skin}{hair}██
       {hair}██  {eye}██{hair}  {hair}██  {eye}██{hair}  {hair}██
       {hair}██  {eye}██{hair}  {hair}██  {eye}██{hair}  {hair}██
        {hair}██{skin}      {skin}{hair}██
         {hair}██{hair}██{hair}██{hair}██{hair}██{hair}██{hair}██{hair}
          {hair}██  {hair}██
         {hair}██{clothes}    {clothes}{hair}██
         {hair}██{clothes}    {clothes}{hair}██
          {hair}██{hair}██{hair}██{hair}██{hair}██";

function colorize($text, $colorName) {
    global $COLORS;
    $code = isset($COLORS[$colorName]) ? $COLORS[$colorName] : 37;
    return "\033[{$code}m{$text}\033[0m";
}

function generateImage($hair, $eyes, $skin, $clothes) {
    global $TEMPLATE;
    $result = $TEMPLATE;
    $result = str_replace('{hair}', colorize('██', $hair), $result);
    $result = str_replace('{eyes}', colorize('██', $eyes), $result);
    $result = str_replace('{skin}', colorize('  ', $skin), $result);
    $result = str_replace('{clothes}', colorize('██', $clothes), $result);
    return $result;
}

$hair = 'magenta';
$eyes = 'cyan';
$skin = 'yellow';
$clothes = 'blue';
$output = null;

$args = array_slice($argv, 1);
for ($i = 0; $i < count($args); $i++) {
    if ($args[$i] == '--hair') $hair = $args[++$i];
    elseif ($args[$i] == '--eyes') $eyes = $args[++$i];
    elseif ($args[$i] == '--skin') $skin = $args[++$i];
    elseif ($args[$i] == '--clothes') $clothes = $args[++$i];
    elseif ($args[$i] == '--output') $output = $args[++$i];
}

echo "🎨 Anime Coloring (PHP)\n";
echo "Цвета: волосы=$hair, глаза=$eyes, кожа=$skin, одежда=$clothes\n\n";
$image = generateImage($hair, $eyes, $skin, $clothes);
echo $image . "\n";

if ($output) {
    file_put_contents($output, $image . "\n");
    echo "💾 Сохранено в $output\n";
}
?>
