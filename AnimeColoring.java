// AnimeColoring.java — Java версия

import java.util.HashMap;
import java.util.Map;

public class AnimeColoring {
    private static final Map<String, Integer> COLORS = new HashMap<>();
    static {
        COLORS.put("black", 30); COLORS.put("red", 31);
        COLORS.put("green", 32); COLORS.put("yellow", 33);
        COLORS.put("blue", 34); COLORS.put("magenta", 35);
        COLORS.put("cyan", 36); COLORS.put("white", 37);
        COLORS.put("gray", 90); COLORS.put("bright_red", 91);
        COLORS.put("bright_green", 92); COLORS.put("bright_yellow", 93);
        COLORS.put("bright_blue", 94); COLORS.put("bright_magenta", 95);
        COLORS.put("bright_cyan", 96); COLORS.put("bright_white", 97);
    }

    private static final String TEMPLATE = 
        "\n          {hair}██{hair}██{hair}██{hair}██{hair}██" +
        "\n        {hair}██{skin}      {skin}{hair}██" +
        "\n       {hair}██  {eye}██{hair}  {hair}██  {eye}██{hair}  {hair}██" +
        "\n       {hair}██  {eye}██{hair}  {hair}██  {eye}██{hair}  {hair}██" +
        "\n        {hair}██{skin}      {skin}{hair}██" +
        "\n         {hair}██{hair}██{hair}██{hair}██{hair}██{hair}██{hair}██{hair}" +
        "\n          {hair}██  {hair}██" +
        "\n         {hair}██{clothes}    {clothes}{hair}██" +
        "\n         {hair}██{clothes}    {clothes}{hair}██" +
        "\n          {hair}██{hair}██{hair}██{hair}██{hair}██";

    private static String colorize(String text, String colorName) {
        Integer code = COLORS.get(colorName);
        if (code == null) code = 37;
        return "\u001B[" + code + "m" + text + "\u001B[0m";
    }

    private static String generateImage(String hair, String eyes, String skin, String clothes) {
        String result = TEMPLATE;
        result = result.replace("{hair}", colorize("██", hair));
        result = result.replace("{eyes}", colorize("██", eyes));
        result = result.replace("{skin}", colorize("  ", skin));
        result = result.replace("{clothes}", colorize("██", clothes));
        return result;
    }

    public static void main(String[] args) {
        String hair = "magenta", eyes = "cyan", skin = "yellow", clothes = "blue";
        String output = null;

        for (int i = 0; i < args.length; i++) {
            if (args[i].equals("--hair")) hair = args[++i];
            else if (args[i].equals("--eyes")) eyes = args[++i];
            else if (args[i].equals("--skin")) skin = args[++i];
            else if (args[i].equals("--clothes")) clothes = args[++i];
            else if (args[i].equals("--output")) output = args[++i];
        }

        System.out.println("🎨 Anime Coloring (Java)");
        System.out.printf("Цвета: волосы=%s, глаза=%s, кожа=%s, одежда=%s\n", hair, eyes, skin, clothes);
        System.out.println();
        String image = generateImage(hair, eyes, skin, clothes);
        System.out.println(image);

        if (output != null) {
            try (java.io.FileWriter fw = new java.io.FileWriter(output)) {
                fw.write(image + "\n");
                System.out.println("💾 Сохранено в " + output);
            } catch (Exception e) {
                System.err.println("Ошибка сохранения: " + e.getMessage());
            }
        }
    }
}
