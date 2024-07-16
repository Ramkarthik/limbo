use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub enum AggFunc {
    Avg,
    Count,
    GroupConcat,
    Max,
    Min,
    StringAgg,
    Sum,
    Total,
}

impl AggFunc {
    pub fn to_string(&self) -> &str {
        match self {
            AggFunc::Avg => "avg",
            AggFunc::Count => "count",
            AggFunc::GroupConcat => "group_concat",
            AggFunc::Max => "max",
            AggFunc::Min => "min",
            AggFunc::StringAgg => "string_agg",
            AggFunc::Sum => "sum",
            AggFunc::Total => "total",
        }
    }
}

#[derive(Debug)]
pub enum SingleRowFunc {
    Coalesce,
}

impl SingleRowFunc {
    pub fn to_string(&self) -> &str {
        match self {
            SingleRowFunc::Coalesce => "coalesce"
        }
    }
}

#[derive(Debug)]
pub enum JsonFunc {
    Json,
}

impl JsonFunc {
    pub fn to_string(&self) -> &str {
        match self {
            JsonFunc::Json => "json"
        }
    }
}

#[derive(Debug)]
pub enum Func {
    Agg(AggFunc),
    SingleRow(SingleRowFunc),
    Json(JsonFunc),
}

impl Func {
    pub fn to_string(&self) -> &str {
        match self {
            Func::Agg(agg) => agg.to_string(),
            Func::SingleRow(sr) => sr.to_string(),
            Func::Json(j) => j.to_string(),
        }
    }
}

impl FromStr for Func {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "avg" => Ok(Func::Agg(AggFunc::Avg)),
            "count" => Ok(Func::Agg(AggFunc::Count)),
            "group_concat" => Ok(Func::Agg(AggFunc::GroupConcat)),
            "max" => Ok(Func::Agg(AggFunc::Max)),
            "min" => Ok(Func::Agg(AggFunc::Min)),
            "string_agg" => Ok(Func::Agg(AggFunc::StringAgg)),
            "sum" => Ok(Func::Agg(AggFunc::Sum)),
            "total" => Ok(Func::Agg(AggFunc::Total)),
            "coalesce" => Ok(Func::SingleRow(SingleRowFunc::Coalesce)),
            "json" => Ok(Func::Json(JsonFunc::Json)),
            _ => Err(()),
        }
    }
}
