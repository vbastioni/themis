// #![allow(non_snake_case)]

use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct MetaCommun {
    pub id: String,
    pub ancien_id: Option<String>,
    pub origine: String,
    pub url: String,
    pub nature: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Theme {
    pub code: String,
    pub libelle: String,
    pub groupe: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Themes {
    pub theme: Option<Vec<Theme>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct MetaAcco {
    pub titre_txt: String,
    pub document_bureautique: String,
    pub numero: String,
    pub siret: String,
    pub code_unite_signataire: String,
    pub date_maj: String,
    pub date_depot: String,
    pub date_texte: String,
    pub date_effet: String,
    pub date_fin: String,
    pub date_diffusion: String,
    pub code_ape: String,
    pub code_idcc: String,
    pub raison_sociale: String,
    pub conforme_version_integrale: String,
    pub secteur: String,
    pub themes: Themes,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct MetaSpec {
    pub meta_acco: MetaAcco,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Meta {
    pub meta_commun: MetaCommun,
    pub meta_spec: MetaSpec,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct TextAcco {
    pub meta: Meta,
}
