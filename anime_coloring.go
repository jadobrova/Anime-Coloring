// anime_coloring.go — Go версия

package main

import (
	"flag"
	"fmt"
	"os"
	"strings"
)

var colors = map[string]int{
	"black": 30, "red": 31, "green": 32, "yellow": 33,
	"blue": 34, "magenta": 35, "cyan": 36, "white": 37,
	"gray": 90, "bright_red": 91, "bright_green": 92,
	"bright_yellow": 93, "bright_blue": 94, "bright_magenta": 95,
	"bright_cyan": 96, "bright_white": 97,
}

const template = `
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
`

func colorize(text, colorName string) string {
	code, ok := colors[colorName]
	if !ok {
		code = 37 // white
	}
	return fmt.Sprintf("\033[%dm%s\033[0m", code, text)
}

func generateImage(hair, eyes, skin, clothes string) string {
	result := template
	result = strings.ReplaceAll(result, "{hair}", colorize("██", hair))
	result = strings.ReplaceAll(result, "{eyes}", colorize("██", eyes))
	result = strings.ReplaceAll(result, "{skin}", colorize("  ", skin))
	result = strings.ReplaceAll(result, "{clothes}", colorize("██", clothes))
	return result
}

func main() {
	hair := flag.String("hair", "magenta", "Цвет волос")
	eyes := flag.String("eyes", "cyan", "Цвет глаз")
	skin := flag.String("skin", "yellow", "Цвет кожи")
	clothes := flag.String("clothes", "blue", "Цвет одежды")
	output := flag.String("output", "", "Сохранить в файл")
	flag.Parse()

	fmt.Println("🎨 Anime Coloring (Go)")
	fmt.Printf("Цвета: волосы=%s, глаза=%s, кожа=%s, одежда=%s\n", *hair, *eyes, *skin, *clothes)
	fmt.Println()
	image := generateImage(*hair, *eyes, *skin, *clothes)
	fmt.Println(image)

	if *output != "" {
		err := os.WriteFile(*output, []byte(image+"\n"), 0644)
		if err == nil {
			fmt.Printf("💾 Сохранено в %s\n", *output)
		}
	}
}
