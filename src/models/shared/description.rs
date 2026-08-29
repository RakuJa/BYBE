use crate::models::creature::creature_metadata::variant_enum::CreatureVariant;
use evalexpr::eval;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, ToSchema, Default, sqlx::Type,
)]
#[sqlx(transparent)]
pub struct Description(pub(crate) String);

#[derive(Default)]
pub struct TagContext {
    pub actor_level: Option<i64>,
    pub creature_variant: CreatureVariant,
    pub variant_damage: Option<i64>,
}

trait ReplaceTag {
    fn replace_tag(self, tag: &str, value: Option<impl ToString>) -> String;
}

impl ReplaceTag for String {
    fn replace_tag(self, tag: &str, value: Option<impl ToString>) -> String {
        match value {
            Some(v) => self.replace(tag, &v.to_string()),
            None => self,
        }
    }
}

impl Description {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }
    pub fn raw(&self) -> &str {
        &self.0
    }

    pub fn resolve(&self, ctx: &TagContext) -> String {
        let solved_tags_description = self
            .0
            .clone()
            .replace_tag("@actor.level", ctx.actor_level)
            .replace_tag(
                "@VariantDamage()",
                ctx.variant_damage.map(|v| format!("@VariantDamage({})", v)),
            );
        resolve_math_tags(&solved_tags_description).unwrap_or(solved_tags_description)
    }
}

fn resolve_math_tags(s: &str) -> Result<String, evalexpr::EvalexprError> {
    let mut result = s.to_owned();
    while let Some((start, end)) = find_next_call(&result) {
        let value = {
            let expr = &result[start..end];
            eval(expr).and_then(|v| {
                v.as_number().map_err(|_| {
                    evalexpr::EvalexprError::CustomMessage(format!(
                        "Expression '{expr}' did not evaluate to a number"
                    ))
                })
            })?
        };
        result.replace_range(start..end, &value.to_string());
    }
    Ok(result)
}

fn find_next_call(s: &str) -> Option<(usize, usize)> {
    const FUNCS: [&str; 3] = ["floor", "ceil", "round"];
    let (start, open) = FUNCS
        .into_iter()
        .filter_map(|func| {
            s.find(func)
                .filter(|&idx| s[idx + func.len()..].starts_with('('))
                .map(|idx| (idx, idx + func.len()))
        })
        .min_by_key(|&(idx, _)| idx)?;
    let close = matching_paren(s, open)?;
    Some((start, close + 1))
}

fn matching_paren(s: &str, open: usize) -> Option<usize> {
    let mut depth = 0;

    for (i, byte) in s.as_bytes()[open..].iter().enumerate() {
        match byte {
            b'(' => depth += 1,
            b')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(open + i);
                }
            }
            _ => {}
        }
    }

    None
}
