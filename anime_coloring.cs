// anime_coloring.cs — C# версия

using System;
using System.Collections.Generic;
using System.IO;

class AnimeColoring
{
    static Dictionary<string, int> COLORS = new Dictionary<string, int>
    {
        {"black", 30}, {"red", 31}, {"green", 32}, {"yellow", 33},
        {"blue", 34}, {"magenta", 35}, {"cyan", 36}, {"white", 37},
        {"gray", 90}, {"bright_red", 91}, {"bright_green", 92},
        {"bright_yellow", 93}, {"bright_blue", 94}, {"bright_magenta", 95},
        {"bright_cyan", 96}, {"bright_white", 97}
    };

    static string TEMPLATE = @"
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

    static string Colorize(string text, string colorName)
    {
        if (!COLORS.TryGetValue(colorName, out int code))
            code = 37;
        return $"\u001B[{code}m{text}\u001B[0m";
    }

    static string GenerateImage(string hair, string eyes, string skin, string clothes)
    {
        string result = TEMPLATE;
        result = result.Replace("{hair}", Colorize("██", hair));
        result = result.Replace("{eyes}", Colorize("██", eyes));
        result = result.Replace("{skin}", Colorize("  ", skin));
        result = result.Replace("{clothes}", Colorize("██", clothes));
        return result;
    }

    static void Main(string[] args)
    {
        string hair = "magenta", eyes = "cyan", skin = "yellow", clothes = "blue";
        string output = null;

        for (int i = 0; i < args.Length; i++)
        {
            if (args[i] == "--hair") hair = args[++i];
            else if (args[i] == "--eyes") eyes = args[++i];
            else if (args[i] == "--skin") skin = args[++i];
            else if (args[i] == "--clothes") clothes = args[++i];
            else if (args[i] == "--output") output = args[++i];
        }

        Console.WriteLine("🎨 Anime Coloring (C#)");
        Console.WriteLine($"Цвета: волосы={hair}, глаза={eyes}, кожа={skin}, одежда={clothes}");
        Console.WriteLine();
        string image = GenerateImage(hair, eyes, skin, clothes);
        Console.WriteLine(image);

        if (output != null)
        {
            File.WriteAllText(output, image + "\n");
            Console.WriteLine($"💾 Сохранено в {output}");
        }
    }
}
