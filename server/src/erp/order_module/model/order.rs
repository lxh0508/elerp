use ahash::HashSet;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use strum::AsRefStr;
use utoipa::{IntoParams, ToSchema};

use crate::{
    erp::util::{eq_or_not, get_sort_col_str, get_sorter_str, in_or_not, like_or_not},
    myhelper::set_to_string,
};

#[derive(
    Debug,
    Serialize,
    Deserialize,
    ToSchema,
    Hash,
    sqlx::Type,
    AsRefStr,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    Copy,
)]
pub enum OrderType {
    StockIn,
    StockOut,
    Return,
    Exchange,
    Calibration,
    CalibrationStrict,
    Verification,
    VerificationStrict,
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    ToSchema,
    Hash,
    sqlx::Type,
    AsRefStr,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    Copy,
    strum::Display,
)]
pub enum OrderPaymentStatus {
    Settled,
    Unsettled,
    PartialSettled,
    None,
}

impl Default for OrderPaymentStatus {
    fn default() -> Self {
        Self::Unsettled
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(
    Debug,
    Serialize,
    Deserialize,
    ToSchema,
    Hash,
    sqlx::Type,
    AsRefStr,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    Copy,
)]
pub enum OrderCurrency {
    CNY,
    HKD,
    USD,
    GBP,
    MYR,
    IDR,
    INR,
    PHP,
    Unknown,
}

impl Default for OrderCurrency {
    fn default() -> Self {
        Self::Unknown
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Order {
    /// Id will generated by the system.
    #[serde(default)]
    pub id: i64,
    /// Id will generated by the system.
    #[serde(default)]
    pub created_by_user_id: i64,
    /// Id will generated by the system.
    #[serde(default)]
    pub updated_by_user_id: i64,
    /// Date will generated by the system.
    #[serde(default)]
    pub date: i64,
    /// Last updated date will generated by the system.
    #[serde(default)]
    pub last_updated_date: i64,
    /// Person in charge will generated by the system.
    #[serde(default)]
    pub person_in_charge_id: i64,
    /// Order status will generated by the system.
    #[serde(default)]
    pub order_category_id: i64,

    /// Order status will generated by the system.
    #[serde(default)]
    pub from_guest_order_id: i64,
    #[serde(default)]
    pub currency: OrderCurrency,
    #[serde(default)]
    pub items: Vec<OrderItem>,
    #[serde(default)]
    pub total_amount: f64,
    #[serde(default)]
    pub total_amount_settled: f64,
    #[serde(default)]
    pub order_payment_status: OrderPaymentStatus,
    #[serde(default)]
    pub warehouse_id: i64,
    #[serde(default)]
    pub person_related_id: i64,
    #[serde(default)]
    pub description: String,
    pub order_type: OrderType,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, IntoParams, FromRow)]
pub struct OrderItem {
    pub sku_id: i64,
    pub quantity: i64,
    pub price: f64,
    #[serde(default)]
    pub exchanged: bool,
}

#[derive(Debug, Deserialize, ToSchema, IntoParams)]
pub struct GetOrdersQuery {
    pub id: Option<i64>,
    pub created_by_user_id: Option<i64>,
    pub updated_by_user_id: Option<i64>,
    pub fuzzy: Option<String>,
    pub warehouse_ids: Option<HashSet<i64>>,
    pub person_related_id: Option<i64>,
    pub person_in_charge_id: Option<i64>,
    pub order_payment_status: Option<HashSet<OrderPaymentStatus>>,
    pub order_type: Option<OrderType>,
    pub order_category_id: Option<i64>,
    pub currency: Option<OrderCurrency>,
    pub date_start: Option<i64>,
    pub date_end: Option<i64>,
    pub last_updated_date_start: Option<i64>,
    pub last_updated_date_end: Option<i64>,
    pub sorters: Option<Vec<String>>,
    pub reverse: Option<HashSet<String>>,
}

impl GetOrdersQuery {
    pub fn empty() -> Self {
        Self {
            id: None,
            created_by_user_id: None,
            updated_by_user_id: None,
            fuzzy: None,
            warehouse_ids: None,
            person_in_charge_id: None,
            person_related_id: None,
            order_payment_status: None,
            order_category_id: None,
            order_type: None,
            currency: None,
            date_start: None,
            date_end: None,
            sorters: None,
            reverse: None,
            last_updated_date_start: None,
            last_updated_date_end: None,
        }
    }
    pub fn get_where_condition(&self) -> String {
        let mut conditions = Vec::with_capacity(5);
        let reverse = self.reverse.as_ref();

        if let Some(v) = &self.id {
            let eq = eq_or_not(reverse, "id");
            conditions.push(format!("orders.id{eq}{v}"));
        }
        if let Some(v) = &self.created_by_user_id {
            let eq = eq_or_not(reverse, "created_by_user_id");
            conditions.push(format!("orders.created_by_user_id{eq}{v}"));
        }
        if let Some(v) = &self.updated_by_user_id {
            let eq = eq_or_not(reverse, "updated_by_user_id");
            conditions.push(format!("orders.updated_by_user_id{eq}{v}"));
        }
        if let Some(v) = &self.fuzzy {
            let eq = like_or_not(reverse, "fuzzy");
            conditions.push(format!("CAST(orders.id AS TEXT) {eq} '%{v}%' OR persons_related.name {eq} '%{v}%' OR persons_in_charge.name {eq} '%{v}%' OR order_status_list.name {eq} '%{v}%' OR warehouses.name {eq} '%{v}%'"));
        }
        if let Some(v) = &self.warehouse_ids {
            let eq = in_or_not(reverse, "warehouse_ids");
            let v = set_to_string(&v, ",");
            conditions.push(format!("orders.warehouse_id{eq}({v})"));
        }
        if let Some(v) = &self.person_related_id {
            let eq = eq_or_not(reverse, "person_related_id");
            conditions.push(format!("orders.person_related_id{eq}{v}"));
        }
        if let Some(v) = &self.person_in_charge_id {
            let eq = eq_or_not(reverse, "person_in_charge_id");
            conditions.push(format!("orders.person_in_charge_id{eq}{v}"));
        }
        if let Some(v) = &self.order_type {
            let eq = eq_or_not(reverse, "order_type");
            conditions.push(format!("orders.order_type{eq}'{}'", v.as_ref()));
        }
        if let Some(v) = &self.order_payment_status {
            let eq = in_or_not(reverse, "order_payment_status");
            let v = set_to_string(v, "','");
            conditions.push(format!("orders.order_payment_status{eq}('{v}')"));
        }
        if let Some(v) = &self.order_category_id {
            let eq = eq_or_not(reverse, "order_category_id");
            conditions.push(format!("orders.order_category_id{eq}{v}"));
        }
        if let Some(v) = &self.currency {
            let eq = eq_or_not(reverse, "currency");
            conditions.push(format!("orders.currency{eq}'{}'", v.as_ref()));
        }
        if let Some(v) = &self.date_start {
            conditions.push(format!("orders.date>={v}"));
        }
        if let Some(v) = &self.date_end {
            conditions.push(format!("orders.date<={v}"));
        }
        if let Some(v) = &self.last_updated_date_start {
            conditions.push(format!("orders.last_updated_date_start>={v}"));
        }
        if let Some(v) = &self.last_updated_date_end {
            conditions.push(format!("orders.last_updated_date_end<={v}"));
        }
        if !conditions.is_empty() {
            let c = conditions.join(" AND ");
            format!("WHERE {c}").into()
        } else {
            "".into()
        }
    }

    pub fn get_order_condition(&self) -> String {
        if self.sorters.is_none() {
            return "".into();
        }
        let mut conditions = vec![];
        for sorter in self.sorters.as_ref().unwrap() {
            let mut col = get_sort_col_str(sorter);
            let sort = get_sorter_str(sorter);
            if col == "warehouse_id" {
                col = format!("warehouse_name {sort}").into();
            } else if col == "person_related_id" {
                col = format!("person_related_name {sort}").into();
            } else if col == "person_in_charge_id" {
                col = format!("person_in_charge_name {sort}").into();
            } else if col == "order_category_id" {
                col = format!("order_status_name {sort}").into();
            } else {
                col = format!("orders.{col} {sort}").into();
            }
            conditions.push(col);
        }
        if !conditions.is_empty() {
            let c = conditions.join(", ");
            format!("ORDER BY {c}").into()
        } else {
            "".into()
        }
    }
}
