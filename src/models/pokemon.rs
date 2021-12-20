use serde::{
    de::{self, IgnoredAny, SeqAccess, Visitor},
    ser::SerializeStruct,
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::marker::PhantomData;

#[derive(Deserialize, Serialize)]
pub struct Language {
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct Description {
    #[serde(rename(deserialize = "flavor_text"))]
    pub text: String,
    pub language: Language,
}

#[derive(Deserialize, Serialize)]
pub struct Habitat {
    pub name: String,
}
#[derive(Deserialize)]
pub struct Pokemon {
    pub name: String,
    #[serde(rename(deserialize = "flavor_text_entries"))]
    #[serde(deserialize_with = "deserialize_description")]
    pub description: Description,
    pub is_legendary: bool,
    pub habitat: Option<Habitat>,
}

impl Pokemon {
    pub fn is_cave_habitat(&self) -> bool {
        match &self.habitat {
            Some(habitat) => habitat.name == "cave",
            _ => false,
        }
    }
    pub fn get_description(&self) -> &str {
        &self.description.text
    }
    pub fn set_description(&mut self, description: &str) {
        self.description.text = description.to_string();
    }
}
/// Deserialize the first English description. The entire sequence
/// is not buffered into memory as it would be if we deserialize to Vec<Description>
/// and then get the first later.
fn deserialize_description<'de, D>(deserializer: D) -> Result<Description, D::Error>
where
    D: Deserializer<'de>,
{
    struct FirstVisitor(PhantomData<fn() -> Description>);

    impl<'de> Visitor<'de> for FirstVisitor {
        type Value = Description;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a nonempty sequence of numbers")
        }

        fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error>
        where
            S: SeqAccess<'de>,
        {
            // Get the first value in the sequence.
            let mut first: Description = seq
                .next_element()?
                .ok_or_else(|| de::Error::custom("no values in seq"))?;
            while let Some(description) = seq.next_element::<Description>()? {
                if first.language.name == "en" {
                    break;
                } else if &description.language.name == "en" {
                    first = description;
                    break;
                }
            }

            // Skip over any remaining elements in the sequence.
            while let Some(IgnoredAny) = seq.next_element()? {
                // ignore
            }
            // Text cleanup
            first.text = first.text.split_whitespace().collect::<Vec<_>>().join(" ");

            Ok(first)
        }
    }

    let visitor = FirstVisitor(PhantomData);
    deserializer.deserialize_seq(visitor)
}

impl Serialize for Pokemon {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Pokemon", 10)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("description", &self.description.text)?;
        state.serialize_field("is_legendary", &self.is_legendary)?;
        if let Some(habitat) = &self.habitat {
            state.serialize_field("habitat", &habitat.name)?;
        } else {
            state.serialize_field("habitat", &())?;
        }
        state.end()
    }
}
