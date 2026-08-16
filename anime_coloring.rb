# anime_coloring.rb — Ruby версия

COLORS = {
  'black' => 30, 'red' => 31, 'green' => 32, 'yellow' => 33,
  'blue' => 34, 'magenta' => 35, 'cyan' => 36, 'white' => 37,
  'gray' => 90, 'bright_red' => 91, 'bright_green' => 92,
  'bright_yellow' => 93, 'bright_blue' => 94, 'bright_magenta' => 95,
  'bright_cyan' => 96, 'bright_white' => 97
}

TEMPLATE = <<~TEXT

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
TEXT

def colorize(text, color_name)
  code = COLORS[color_name] || 37
  "\e[#{code}m#{text}\e[0m"
end

def generate_image(hair, eyes, skin, clothes)
  result = TEMPLATE.dup
  result.gsub!('{hair}', colorize('██', hair))
  result.gsub!('{eyes}', colorize('██', eyes))
  result.gsub!('{skin}', colorize('  ', skin))
  result.gsub!('{clothes}', colorize('██', clothes))
  result
end

def main
  hair = 'magenta'
  eyes = 'cyan'
  skin = 'yellow'
  clothes = 'blue'
  output = nil

  args = ARGV
  i = 0
  while i < args.size
    case args[i]
    when '--hair' then hair = args[i+1]; i += 2
    when '--eyes' then eyes = args[i+1]; i += 2
    when '--skin' then skin = args[i+1]; i += 2
    when '--clothes' then clothes = args[i+1]; i += 2
    when '--output' then output = args[i+1]; i += 2
    else i += 1
    end
  end

  puts '🎨 Anime Coloring (Ruby)'
  puts "Цвета: волосы=#{hair}, глаза=#{eyes}, кожа=#{skin}, одежда=#{clothes}"
  puts
  image = generate_image(hair, eyes, skin, clothes)
  puts image

  if output
    File.write(output, image + "\n")
    puts "💾 Сохранено в #{output}"
  end
end

main if __FILE__ == $0
