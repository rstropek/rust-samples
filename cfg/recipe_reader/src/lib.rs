#[cfg(any(feature = "json", feature = "yaml"))]
use std::{fs::File, io::Read};

use anyhow::Result;
use serde::Deserialize;

#[cfg(test)]
use mockall::automock;

#[derive(Debug, Deserialize)]
pub struct Recipe {
    pub title: String,
    pub ingredients: Option<Vec<String>>,
    pub steps: Option<Vec<String>>,
}

#[cfg_attr(test, automock)]
pub trait RecipeReader {
    fn read_from_file(&self, path: &str) -> Result<Recipe>;
}

#[cfg(feature = "json")]
mod json
{
    use super::*;

    pub struct JsonRecipeReader();

    impl RecipeReader for JsonRecipeReader {
        fn read_from_file(&self, path: &str) -> Result<Recipe> {
            let mut contents = String::new();
            File::open(path)?.read_to_string(&mut contents)?;
            Ok(serde_json::from_str(&contents)?)
        }
    }
}

#[cfg(feature = "yaml")]
mod yaml
{
    use super::*;
    
    pub struct YamlRecipeReader();

    impl RecipeReader for YamlRecipeReader {
        fn read_from_file(&self, path: &str) -> Result<Recipe> {
            let mut contents = String::new();
            File::open(path)?.read_to_string(&mut contents)?;
            Ok(serde_yaml::from_str(&contents)?)
        }
    }
}

pub struct RecipeMarkdownFactory();

impl RecipeMarkdownFactory {
    pub fn file_to_markdown(&self, path: &str, reader: impl RecipeReader) -> Result<String> {
        let recipe = reader.read_from_file(path)?;
        let mut markdown = String::new();
        markdown.push_str(&format!("# {}\n\n", recipe.title));
        if let Some(i) = recipe.ingredients {
            markdown.push_str("## Ingredients\n\n");
            for ingredient in i {
                markdown.push_str(&format!("* {ingredient}\n"));
            }
            markdown.push('\n');
        }

        if let Some(s) = recipe.steps{
            markdown.push_str("## Steps\n\n");
            for (i, step) in s.iter().enumerate() {
                markdown.push_str(&format!("{}. {step}\n", i + 1));
            }
        }

        Ok(markdown)
    }
}

#[cfg(test)]
mod tests {
    use mockall::predicate::eq;

    use super::*;

    #[cfg(feature = "json")]
    use super::json::*;
    #[cfg(feature = "yaml")]
    use super::yaml::*;

    #[cfg(any(feature = "json", feature = "yaml"))]
    fn get_expected_markdown() -> String {
        let mut markdown = String::new();
        File::open("caesar_expected.md").unwrap().read_to_string(&mut markdown).unwrap();
        markdown
    }

    #[cfg(feature = "json")]
    #[test]
    fn json_caesar_salad_to_markdown() {
        let reader = JsonRecipeReader();
        let markdown_factory = RecipeMarkdownFactory();
        let markdown = markdown_factory.file_to_markdown("caesar_salad.json", reader).unwrap();

        assert_eq!(markdown, get_expected_markdown());
    }

    #[cfg(feature = "yaml")]
    #[test]
    fn yaml_caesar_salad_to_markdown() {
        let reader = YamlRecipeReader();
        let markdown_factory = RecipeMarkdownFactory();
        let markdown = markdown_factory.file_to_markdown("caesar_salad.yaml", reader).unwrap();

        assert_eq!(markdown, get_expected_markdown());
    }

    #[test]
    fn no_ingredients() {
        let mut reader_mock = MockRecipeReader::new();
        reader_mock
            .expect_read_from_file()
            .with(eq("no_ingredients.json"))
            .returning(|_| {
                Ok(Recipe {
                    title: "No Ingredients".to_string(),
                    ingredients: None,
                    steps: Some(vec!["Step 1".to_string(), "Step 2".to_string()]),
                })
            });
        
        let markdown_factory = RecipeMarkdownFactory();
        let markdown = markdown_factory.file_to_markdown("no_ingredients.json", reader_mock).unwrap();

        assert!(!markdown.contains("## Ingredients"));
    }
}
