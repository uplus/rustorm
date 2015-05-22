use chrono::naive::date::NaiveDate;
use chrono::naive::datetime::NaiveDateTime;

pub struct AAsset {
	/// primary
	/// not nullable 
	pub a_asset_id:f64,
	pub a_asset_createdate:Option<NaiveDateTime>,
	/// not nullable 
	pub a_asset_group_id:f64,
	pub a_asset_revaldate:Option<NaiveDateTime>,
	pub a_parent_asset_id:Option<f64>,
	pub a_qty_current:Option<f64>,
	pub a_qty_original:Option<f64>,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_user_id:Option<f64>,
	pub assetdepreciationdate:Option<NaiveDateTime>,
	pub assetdisposaldate:Option<NaiveDateTime>,
	pub assetservicedate:Option<NaiveDateTime>,
	pub c_bpartner_id:Option<f64>,
	pub c_bpartner_location_id:Option<f64>,
	pub c_bpartnersr_id:Option<f64>,
	pub c_location_id:Option<f64>,
	pub c_project_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub guaranteedate:Option<NaiveDateTime>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isdepreciated:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isdisposed:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isfullydepreciated:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isinposession:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isowned:String,
	pub lastmaintenancedate:Option<NaiveDateTime>,
	pub lastmaintenancenote:Option<String>,
	pub lastmaintenanceunit:Option<f64>,
	pub lastmaintenanceuseunit:Option<f64>,
	pub lastmaintenencedate:Option<NaiveDateTime>,
	pub lease_bpartner_id:Option<f64>,
	pub leaseterminationdate:Option<NaiveDateTime>,
	pub lifeuseunits:Option<f64>,
	pub locationcomment:Option<String>,
	pub lot:Option<String>,
	/// defaults to: 0
	pub m_attributesetinstance_id:Option<f64>,
	pub m_inoutline_id:Option<f64>,
	pub m_locator_id:Option<f64>,
	pub m_product_id:Option<f64>,
	/// not nullable 
	pub name:String,
	pub nextmaintenanceuseunit:Option<f64>,
	pub nextmaintenencedate:Option<NaiveDateTime>,
	pub nextmaintenenceunit:Option<f64>,
	/// defaults to: 'N'::bpchar
	pub processing:Option<String>,
	pub qty:Option<f64>,
	pub serno:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub uselifemonths:Option<f64>,
	pub uselifeyears:Option<f64>,
	pub useunits:Option<f64>,
	/// not nullable 
	pub value:String,
	pub versionno:Option<String>,
	/// has one
	pub a_asset_group_id_a_asset_group:Option<AAssetGroup>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub m_locator_id_m_locator:Option<MLocator>,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub c_bpartner_location_id_c_bpartner_location:Option<CBpartnerLocation>,
	/// has one
	pub c_location_id_c_location:Option<CLocation>,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
	/// has one
	pub m_attributesetinstance_id_m_attributesetinstance:Option<MAttributesetinstance>,
	/// has one
	pub c_project_id_c_project:Option<CProject>,
	/// has one
	pub c_bpartnersr_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub m_inoutline_id_m_inoutline:Option<MInoutline>,
	/// has one
	pub lease_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one, self referential
	pub a_parent_asset_id_a_asset:Option<Box<AAsset>>,
	/// has many
	pub a_asset:Option<Vec<AAsset>>,
	/// has many
	pub a_asset_acct:Option<Vec<AAssetAcct>>,
	/// has many
	pub a_asset_addition:Option<Vec<AAssetAddition>>,
	/// has many
	pub a_asset_change:Option<Vec<AAssetChange>>,
	/// has many
	pub a_asset_delivery:Option<Vec<AAssetDelivery>>,
	/// has many
	pub a_asset_disposed:Option<Vec<AAssetDisposed>>,
	/// has many
	pub a_asset_retirement:Option<Vec<AAssetRetirement>>,
	/// has many
	pub a_asset_use:Option<Vec<AAssetUse>>,
	/// has many
	pub a_depreciation_build:Option<Vec<ADepreciationBuild>>,
	/// has many
	pub a_depreciation_forecast:Option<Vec<ADepreciationForecast>>,
	/// has many
	pub a_registration:Option<Vec<ARegistration>>,
	/// has many
	pub ad_issue:Option<Vec<AdIssue>>,
	/// has many
	pub c_invoiceline:Option<Vec<CInvoiceline>>,
	/// has many
	pub fact_acct:Option<Vec<FactAcct>>,
	/// has many
	pub gl_journalline:Option<Vec<GlJournalline>>,
	/// has many
	pub m_operationresource:Option<Vec<MOperationresource>>,
	/// has many
	pub pp_order_node_asset:Option<Vec<PpOrderNodeAsset>>,
	/// has many
	pub pp_wf_node_asset:Option<Vec<PpWfNodeAsset>>,
	/// has many
	pub r_issueproject:Option<Vec<RIssueproject>>,
	/// has many
	pub r_issuesystem:Option<Vec<RIssuesystem>>,
	/// has many
	pub r_request:Option<Vec<RRequest>>,
	/// has many
	pub r_requestaction:Option<Vec<RRequestaction>>,
}

pub struct AAssetAcct {
	/// primary
	/// not nullable 
	pub a_asset_acct_id:f64,
	pub a_accumdepreciation_acct:Option<f64>,
	pub a_asset_acct:Option<f64>,
	/// not nullable 
	pub a_asset_id:f64,
	pub a_asset_spread_id:Option<f64>,
	pub a_depreciation_acct:Option<f64>,
	/// not nullable 
	pub a_depreciation_conv_id:f64,
	/// not nullable 
	pub a_depreciation_id:f64,
	pub a_depreciation_manual_amount:Option<f64>,
	/// defaults to: 'PR'::character varying
	pub a_depreciation_manual_period:Option<String>,
	/// not nullable 
	pub a_depreciation_method_id:f64,
	pub a_depreciation_table_header_id:Option<f64>,
	pub a_depreciation_variable_perc:Option<f64>,
	pub a_disposal_gain:Option<String>,
	pub a_disposal_loss:Option<String>,
	pub a_disposal_revenue:Option<String>,
	/// not nullable 
	pub a_period_end:f64,
	/// not nullable 
	pub a_period_start:f64,
	pub a_reval_accumdep_offset_cur:Option<String>,
	pub a_reval_accumdep_offset_prior:Option<String>,
	/// defaults to: 'DFT'::character varying
	pub a_reval_cal_method:Option<String>,
	pub a_reval_cost_offset:Option<String>,
	pub a_reval_cost_offset_prior:Option<String>,
	pub a_reval_depexp_offset:Option<String>,
	/// not nullable 
	pub a_salvage_value:f64,
	/// not nullable 
	pub a_split_percent:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_acctschema_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub postingtype:String,
	/// defaults to: 'Y'::bpchar
	pub processing:Option<String>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub a_asset_id_a_asset:Option<AAsset>,
	/// has one
	pub c_acctschema_id_c_acctschema:Option<CAcctschema>,
	/// has one
	pub a_depreciation_id_a_depreciation:Option<ADepreciation>,
	/// has one
	pub a_depreciation_table_header_id_a_depreciation_table_header:Option<ADepreciationTableHeader>,
	/// has one
	pub a_depreciation_method_id_a_depreciation_method:Option<ADepreciationMethod>,
	/// has one
	pub a_depreciation_conv_id_a_depreciation_convention:Option<ADepreciationConvention>,
	/// has one
	pub a_asset_spread_id_a_asset_spread:Option<AAssetSpread>,
}

pub struct AAssetAddition {
	/// primary
	/// not nullable 
	pub a_asset_addition_id:f64,
	/// not nullable 
	pub a_asset_id:f64,
	pub a_capvsexp:Option<String>,
	pub a_qty_current:Option<f64>,
	pub a_sourcetype:Option<String>,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub assetvalueamt:f64,
	pub c_invoice_id:Option<f64>,
	pub c_invoiceline_id:Option<f64>,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub documentno:Option<String>,
	pub gl_journalbatch_id:Option<f64>,
	/// not nullable 
	pub isactive:String,
	pub line:Option<f64>,
	pub m_inoutline_id:Option<f64>,
	pub postingtype:Option<String>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub a_asset_id_a_asset:Option<AAsset>,
	/// has one
	pub c_invoiceline_id_c_invoiceline:Option<CInvoiceline>,
	/// has one
	pub gl_journalbatch_id_gl_journalbatch:Option<GlJournalbatch>,
	/// has one
	pub c_invoice_id_c_invoice:Option<CInvoice>,
	/// has many
	pub a_asset_change:Option<Vec<AAssetChange>>,
}

pub struct AAssetChange {
	/// primary
	/// not nullable 
	pub a_asset_change_id:f64,
	pub a_accumdepreciation_acct:Option<f64>,
	pub a_asset_acct:Option<f64>,
	pub a_asset_acct_id:Option<f64>,
	pub a_asset_addition_id:Option<f64>,
	pub a_asset_createdate:Option<NaiveDateTime>,
	/// not nullable 
	pub a_asset_id:f64,
	pub a_asset_retirement_id:Option<f64>,
	pub a_asset_revaldate:Option<NaiveDateTime>,
	pub a_asset_spread_type:Option<String>,
	pub a_depreciation_acct:Option<f64>,
	pub a_depreciation_calc_type:Option<String>,
	pub a_depreciation_manual_amount:Option<f64>,
	pub a_depreciation_manual_period:Option<String>,
	pub a_depreciation_table_header_id:Option<f64>,
	pub a_depreciation_variable_perc:Option<f64>,
	pub a_disposal_loss:Option<String>,
	pub a_disposal_revenue:Option<String>,
	pub a_parent_asset_id:Option<f64>,
	pub a_period_end:Option<f64>,
	pub a_period_start:Option<f64>,
	pub a_qty_current:Option<f64>,
	pub a_qty_original:Option<f64>,
	pub a_reval_accumdep_offset_cur:Option<f64>,
	pub a_reval_accumdep_offset_prior:Option<f64>,
	pub a_reval_cal_method:Option<String>,
	pub a_reval_cost_offset:Option<f64>,
	pub a_reval_cost_offset_prior:Option<f64>,
	pub a_reval_depexp_offset:Option<f64>,
	pub a_salvage_value:Option<f64>,
	pub a_split_percent:Option<f64>,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_user_id:Option<f64>,
	pub assetaccumdepreciationamt:Option<f64>,
	pub assetbookvalueamt:Option<f64>,
	pub assetdepreciationdate:Option<NaiveDateTime>,
	pub assetdisposaldate:Option<NaiveDateTime>,
	pub assetmarketvalueamt:Option<f64>,
	pub assetservicedate:Option<NaiveDateTime>,
	pub assetvalueamt:Option<f64>,
	pub c_acctschema_id:Option<f64>,
	pub c_bpartner_id:Option<f64>,
	pub c_bpartner_location_id:Option<f64>,
	pub c_location_id:Option<f64>,
	pub c_validcombination_id:Option<f64>,
	pub changeamt:Option<f64>,
	pub changedate:Option<NaiveDateTime>,
	/// not nullable 
	pub changetype:String,
	pub conventiontype:Option<String>,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub dateacct:Option<NaiveDateTime>,
	pub depreciationtype:Option<String>,
	/// not nullable 
	pub isactive:String,
	pub isdepreciated:Option<String>,
	pub isdisposed:Option<String>,
	pub isfullydepreciated:Option<String>,
	pub isinposession:Option<String>,
	pub isowned:Option<String>,
	pub lifeuseunits:Option<f64>,
	pub lot:Option<String>,
	pub postingtype:Option<String>,
	pub serno:Option<String>,
	/// not nullable 
	pub textdetails:String,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub uselifemonths:Option<f64>,
	pub uselifeyears:Option<f64>,
	pub useunits:Option<f64>,
	pub versionno:Option<String>,
	/// has one
	pub a_asset_id_a_asset:Option<AAsset>,
	/// has one
	pub a_asset_retirement_id_a_asset_retirement:Option<AAssetRetirement>,
	/// has one
	pub a_asset_addition_id_a_asset_addition:Option<AAssetAddition>,
	/// has one
	pub a_parent_asset_id_a_asset:Option<AAsset>,
	/// has one
	pub a_depreciation_table_header_id_a_depreciation_table_header:Option<ADepreciationTableHeader>,
	/// has one
	pub c_acctschema_id_c_acctschema:Option<CAcctschema>,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
	/// has one
	pub c_location_id_c_location:Option<CLocation>,
	/// has one
	pub c_bpartner_location_id_c_bpartner_location:Option<CBpartnerLocation>,
	/// has many
	pub a_asset_change_amt:Option<Vec<AAssetChangeAmt>>,
}

pub struct AAssetChangeAmt {
	/// primary
	/// not nullable 
	pub a_asset_change_id:f64,
	/// primary
	/// not nullable 
	pub c_acctschema_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: 0
	/// not nullable 
	pub assetaccumdepreciationamt:f64,
	/// defaults to: 0
	/// not nullable 
	pub assetbookvalueamt:f64,
	/// defaults to: 0
	/// not nullable 
	pub assetmarketvalueamt:f64,
	/// defaults to: 0
	/// not nullable 
	pub assetvalueamt:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub a_asset_change_id_a_asset_change:Option<AAssetChange>,
	/// has one
	pub c_acctschema_id_c_acctschema:Option<CAcctschema>,
}

pub struct AAssetDelivery {
	/// primary
	/// not nullable 
	pub a_asset_delivery_id:f64,
	/// not nullable 
	pub a_asset_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_user_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub deliveryconfirmation:Option<String>,
	pub description:Option<String>,
	pub email:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub lot:Option<String>,
	pub m_inoutline_id:Option<f64>,
	pub m_productdownload_id:Option<f64>,
	pub messageid:Option<String>,
	/// not nullable 
	pub movementdate:NaiveDateTime,
	pub referrer:Option<String>,
	pub remote_addr:Option<String>,
	pub remote_host:Option<String>,
	pub serno:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub url:Option<String>,
	pub versionno:Option<String>,
	/// has one
	pub a_asset_id_a_asset:Option<AAsset>,
	/// has one
	pub m_inoutline_id_m_inoutline:Option<MInoutline>,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
	/// has one
	pub m_productdownload_id_m_productdownload:Option<MProductdownload>,
}

pub struct AAssetDisposed {
	/// primary
	/// not nullable 
	pub a_asset_disposed_id:f64,
	/// not nullable 
	pub a_asset_id:f64,
	pub a_asset_trade_id:Option<f64>,
	/// not nullable 
	pub a_disposed_date:NaiveDateTime,
	/// not nullable 
	pub a_disposed_method:String,
	/// not nullable 
	pub a_disposed_reason:String,
	pub a_proceeds:Option<f64>,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_period_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub dateacct:NaiveDateTime,
	/// not nullable 
	pub datedoc:NaiveDateTime,
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub processed:String,
	/// not nullable 
	pub processing:String,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_period_id_c_period:Option<CPeriod>,
	/// has one
	pub a_asset_trade_id_a_asset:Option<AAsset>,
}

pub struct AAssetGroup {
	/// primary
	/// not nullable 
	pub a_asset_group_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub iscreateasactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isdepreciated:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isoneassetperuom:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isowned:String,
	/// defaults to: 'N'::bpchar
	pub istrackissues:Option<String>,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has many
	pub a_asset:Option<Vec<AAsset>>,
	/// has many
	pub a_asset_group_acct:Option<Vec<AAssetGroupAcct>>,
	/// has many
	pub c_invoiceline:Option<Vec<CInvoiceline>>,
	/// has many
	pub gl_journalline:Option<Vec<GlJournalline>>,
	/// has many
	pub i_asset:Option<Vec<IAsset>>,
	/// has many
	pub m_product_category:Option<Vec<MProductCategory>>,
}

pub struct AAssetGroupAcct {
	/// primary
	/// not nullable 
	pub a_asset_group_acct_id:f64,
	pub a_accumdepreciation_acct:Option<f64>,
	pub a_asset_acct:Option<f64>,
	/// not nullable 
	pub a_asset_group_id:f64,
	pub a_asset_spread_type:Option<String>,
	pub a_depreciation_acct:Option<f64>,
	/// not nullable 
	pub a_depreciation_calc_type:String,
	pub a_depreciation_id:Option<f64>,
	pub a_depreciation_manual_amount:Option<f64>,
	/// defaults to: 'PR'::character varying
	pub a_depreciation_manual_period:Option<String>,
	pub a_depreciation_table_header_id:Option<f64>,
	pub a_depreciation_variable_perc:Option<f64>,
	pub a_disposal_gain:Option<String>,
	pub a_disposal_loss:Option<String>,
	pub a_disposal_revenue:Option<String>,
	pub a_reval_accumdep_offset_cur:Option<String>,
	pub a_reval_accumdep_offset_prior:Option<String>,
	/// defaults to: 'DFT'::character varying
	pub a_reval_cal_method:Option<String>,
	pub a_reval_cost_offset:Option<String>,
	pub a_reval_cost_offset_prior:Option<String>,
	pub a_reval_depexp_offset:Option<String>,
	/// not nullable 
	pub a_split_percent:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_acctschema_id:f64,
	/// not nullable 
	pub conventiontype:String,
	/// not nullable 
	pub created:NaiveDateTime,
	pub createdby:Option<f64>,
	/// not nullable 
	pub depreciationtype:String,
	pub isactive:Option<String>,
	/// not nullable 
	pub postingtype:String,
	pub processing:Option<String>,
	/// not nullable 
	pub updated:NaiveDateTime,
	pub updatedby:Option<f64>,
	pub uselifemonths:Option<f64>,
	pub uselifeyears:Option<f64>,
	/// has one
	pub a_asset_group_id_a_asset_group:Option<AAssetGroup>,
	/// has one
	pub c_acctschema_id_c_acctschema:Option<CAcctschema>,
	/// has one
	pub a_depreciation_id_a_depreciation:Option<ADepreciation>,
	/// has one
	pub a_depreciation_table_header_id_a_depreciation_table_header:Option<ADepreciationTableHeader>,
}

pub struct AAssetInfoFin {
	/// primary
	/// not nullable 
	pub a_asset_info_fin_id:f64,
	/// not nullable 
	pub a_asset_id:f64,
	pub a_contract_date:Option<NaiveDateTime>,
	pub a_due_on:Option<String>,
	pub a_expired_date:Option<NaiveDateTime>,
	pub a_finance_meth:Option<String>,
	pub a_monthly_payment:Option<f64>,
	pub a_purchase_option:Option<String>,
	pub a_purchase_option_credit:Option<f64>,
	pub a_purchase_option_credit_per:Option<f64>,
	pub a_purchase_price:Option<f64>,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub c_bpartner_id:Option<f64>,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub isactive:String,
	pub textmsg:Option<String>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
}

pub struct AAssetInfoIns {
	/// primary
	/// not nullable 
	pub a_asset_info_ins_id:f64,
	/// not nullable 
	pub a_asset_id:f64,
	pub a_ins_premium:Option<f64>,
	pub a_ins_value:Option<f64>,
	pub a_insurance_co:Option<String>,
	pub a_policy_no:Option<String>,
	pub a_renewal_date:Option<NaiveDateTime>,
	pub a_replace_cost:Option<f64>,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub isactive:String,
	pub text:Option<String>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
}

pub struct AAssetInfoLic {
	/// primary
	/// not nullable 
	pub a_asset_info_lic_id:f64,
	/// not nullable 
	pub a_asset_id:f64,
	pub a_issuing_agency:Option<String>,
	pub a_license_fee:Option<f64>,
	pub a_license_no:Option<String>,
	pub a_renewal_date:Option<NaiveDateTime>,
	pub a_state:Option<String>,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub isactive:String,
	pub text:Option<String>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
}

pub struct AAssetInfoOth {
	/// primary
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub a_asset_id:f64,
	/// not nullable 
	pub a_asset_info_oth_id:f64,
	pub a_user1:Option<String>,
	pub a_user10:Option<String>,
	pub a_user11:Option<String>,
	pub a_user12:Option<String>,
	pub a_user13:Option<String>,
	pub a_user14:Option<String>,
	pub a_user15:Option<String>,
	pub a_user2:Option<String>,
	pub a_user3:Option<String>,
	pub a_user4:Option<String>,
	pub a_user5:Option<String>,
	pub a_user6:Option<String>,
	pub a_user7:Option<String>,
	pub a_user8:Option<String>,
	pub a_user9:Option<String>,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub isactive:String,
	pub text:Option<String>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
}

pub struct AAssetInfoTax {
	/// primary
	/// not nullable 
	pub a_asset_info_tax_id:f64,
	/// not nullable 
	pub a_asset_id:f64,
	pub a_finance_meth:Option<String>,
	pub a_investment_cr:Option<f64>,
	pub a_new_used:Option<String>,
	pub a_state:Option<String>,
	pub a_tax_entity:Option<String>,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub isactive:String,
	pub textmsg:Option<String>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
}

pub struct AAssetRetirement {
	/// primary
	/// not nullable 
	pub a_asset_retirement_id:f64,
	/// not nullable 
	pub a_asset_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: 0
	/// not nullable 
	pub assetmarketvalueamt:f64,
	/// defaults to: 0
	/// not nullable 
	pub assetvalueamt:f64,
	pub c_invoiceline_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub a_asset_id_a_asset:Option<AAsset>,
	/// has one
	pub c_invoiceline_id_c_invoiceline:Option<CInvoiceline>,
	/// has many
	pub a_asset_change:Option<Vec<AAssetChange>>,
}

pub struct AAssetRevalEntry {
	/// primary
	/// not nullable 
	pub a_asset_reval_entry_id:f64,
	/// not nullable 
	pub a_effective_date:NaiveDateTime,
	/// not nullable 
	pub a_rev_code:String,
	/// not nullable 
	pub a_reval_cal_method:String,
	/// not nullable 
	pub a_reval_effective_date:String,
	/// not nullable 
	pub a_reval_multiplier:String,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub c_acctschema_id:Option<f64>,
	/// not nullable 
	pub c_currency_id:f64,
	pub c_doctype_id:Option<f64>,
	pub c_period_id:Option<f64>,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub dateacct:Option<NaiveDateTime>,
	pub datedoc:Option<NaiveDateTime>,
	/// not nullable 
	pub description:String,
	/// not nullable 
	pub documentno:String,
	pub gl_category_id:Option<f64>,
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub postingtype:String,
	/// not nullable 
	pub processed:String,
	/// not nullable 
	pub processing:String,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_currency_id_c_currency:Option<CCurrency>,
	/// has one
	pub c_period_id_c_period:Option<CPeriod>,
	/// has one
	pub gl_category_id_gl_category:Option<GlCategory>,
	/// has one
	pub c_doctype_id_c_doctype:Option<CDoctype>,
	/// has one
	pub c_acctschema_id_c_acctschema:Option<CAcctschema>,
}

pub struct AAssetRevalIndex {
	/// primary
	/// not nullable 
	pub a_asset_reval_index_id:f64,
	/// not nullable 
	pub a_effective_date:NaiveDateTime,
	/// not nullable 
	pub a_reval_code:String,
	/// not nullable 
	pub a_reval_multiplier:String,
	/// not nullable 
	pub a_reval_rate:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
}

pub struct AAssetSplit {
	/// primary
	/// not nullable 
	pub a_asset_split_id:f64,
	pub a_amount_split:Option<f64>,
	/// not nullable 
	pub a_asset_acct_id:f64,
	pub a_asset_cost:Option<f64>,
	/// not nullable 
	pub a_asset_id:f64,
	pub a_asset_id_to:Option<String>,
	/// not nullable 
	pub a_depreciation_workfile_id:f64,
	pub a_percent_original:Option<f64>,
	pub a_percent_split:Option<f64>,
	/// not nullable 
	pub a_qty_current:f64,
	pub a_qty_split:Option<f64>,
	/// not nullable 
	pub a_split_type:String,
	/// not nullable 
	pub a_transfer_balance_is:String,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_period_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub dateacct:NaiveDateTime,
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub postingtype:String,
	/// not nullable 
	pub processed:String,
	/// not nullable 
	pub processing:String,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_period_id_c_period:Option<CPeriod>,
}

pub struct AAssetSpread {
	/// primary
	/// not nullable 
	pub a_asset_spread_id:f64,
	pub a_asset_spread_type:Option<String>,
	/// not nullable 
	pub a_period_1:f64,
	/// not nullable 
	pub a_period_10:f64,
	/// not nullable 
	pub a_period_11:f64,
	/// not nullable 
	pub a_period_12:f64,
	/// not nullable 
	pub a_period_13:f64,
	/// not nullable 
	pub a_period_14:f64,
	/// not nullable 
	pub a_period_2:f64,
	/// not nullable 
	pub a_period_3:f64,
	/// not nullable 
	pub a_period_4:f64,
	/// not nullable 
	pub a_period_5:f64,
	/// not nullable 
	pub a_period_6:f64,
	/// not nullable 
	pub a_period_7:f64,
	/// not nullable 
	pub a_period_8:f64,
	/// not nullable 
	pub a_period_9:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has many
	pub a_asset_acct:Option<Vec<AAssetAcct>>,
}

pub struct AAssetTransfer {
	/// primary
	/// not nullable 
	pub a_asset_transfer_id:f64,
	pub a_accumdepreciation_acct:Option<f64>,
	/// not nullable 
	pub a_accumdepreciation_acct_new:String,
	pub a_accumdepreciation_acct_str:Option<String>,
	pub a_asset_acct:Option<f64>,
	pub a_asset_acct_id:Option<f64>,
	/// not nullable 
	pub a_asset_acct_new:String,
	pub a_asset_acct_str:Option<String>,
	pub a_asset_id:Option<f64>,
	pub a_depreciation_acct:Option<f64>,
	/// not nullable 
	pub a_depreciation_acct_new:String,
	pub a_depreciation_acct_str:Option<String>,
	pub a_disposal_loss:Option<f64>,
	/// not nullable 
	pub a_disposal_loss_new:String,
	pub a_disposal_loss_str:Option<String>,
	pub a_disposal_revenue:Option<f64>,
	/// not nullable 
	pub a_disposal_revenue_new:String,
	pub a_disposal_revenue_str:Option<String>,
	/// not nullable 
	pub a_period_end:f64,
	/// not nullable 
	pub a_period_start:f64,
	/// not nullable 
	pub a_split_percent:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub a_transfer_balance:String,
	/// not nullable 
	pub a_transfer_balance_is:String,
	pub ad_client_id:Option<f64>,
	pub ad_org_id:Option<f64>,
	pub c_acctschema_id:Option<f64>,
	pub c_period_id:Option<f64>,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub dateacct:NaiveDateTime,
	/// not nullable 
	pub isactive:String,
	pub postingtype:Option<String>,
	/// not nullable 
	pub processed:String,
	/// not nullable 
	pub processing:String,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_period_id_c_period:Option<CPeriod>,
	/// has one
	pub c_acctschema_id_c_acctschema:Option<CAcctschema>,
}

pub struct AAssetUse {
	/// primary
	/// not nullable 
	pub a_asset_use_id:f64,
	/// not nullable 
	pub a_asset_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub usedate:NaiveDateTime,
	/// not nullable 
	pub useunits:f64,
	/// has one
	pub a_asset_id_a_asset:Option<AAsset>,
}

pub struct ADepreciation {
	/// primary
	/// not nullable 
	pub a_depreciation_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub depreciationtype:String,
	pub description:Option<String>,
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub processed:String,
	pub script:Option<String>,
	pub text:Option<String>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has many
	pub a_asset_acct:Option<Vec<AAssetAcct>>,
	/// has many
	pub a_asset_group_acct:Option<Vec<AAssetGroupAcct>>,
}

pub struct ADepreciationBuild {
	/// primary
	/// not nullable 
	pub a_depreciation_build_id:f64,
	pub a_end_asset_id:Option<f64>,
	pub a_start_asset_id:Option<f64>,
	pub ad_client_id:Option<f64>,
	pub ad_org_id:Option<f64>,
	pub c_period_id:Option<f64>,
	pub created:Option<NaiveDateTime>,
	pub createdby:Option<f64>,
	pub dateacct:Option<NaiveDateTime>,
	pub datedoc:Option<NaiveDateTime>,
	pub isactive:Option<String>,
	pub periodno:Option<f64>,
	/// defaults to: 'A'::bpchar
	pub postingtype:Option<String>,
	pub processed:Option<String>,
	pub processing:Option<String>,
	pub updated:Option<NaiveDateTime>,
	pub updatedby:Option<f64>,
	/// has one
	pub a_end_asset_id_a_asset:Option<AAsset>,
	/// has one
	pub c_period_id_c_period:Option<CPeriod>,
	/// has one
	pub a_start_asset_id_a_asset:Option<AAsset>,
}

pub struct ADepreciationConvention {
	/// primary
	/// not nullable 
	pub a_depreciation_convention_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub conventiontype:Option<String>,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// not nullable 
	pub isactive:String,
	pub name:Option<String>,
	/// not nullable 
	pub processed:String,
	pub textmsg:Option<String>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has many
	pub a_asset_acct:Option<Vec<AAssetAcct>>,
}

pub struct ADepreciationEntry {
	/// primary
	/// not nullable 
	pub a_depreciation_entry_id:f64,
	/// not nullable 
	pub a_entry_type:String,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_acctschema_id:f64,
	/// not nullable 
	pub c_currency_id:f64,
	/// not nullable 
	pub c_doctype_id:f64,
	/// not nullable 
	pub c_period_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub dateacct:NaiveDateTime,
	/// not nullable 
	pub datedoc:NaiveDateTime,
	/// not nullable 
	pub description:String,
	/// not nullable 
	pub documentno:String,
	/// not nullable 
	pub gl_category_id:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'A'::bpchar
	/// not nullable 
	pub postingtype:String,
	/// not nullable 
	pub processed:String,
	/// not nullable 
	pub processing:String,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_currency_id_c_currency:Option<CCurrency>,
	/// has one
	pub c_period_id_c_period:Option<CPeriod>,
	/// has one
	pub gl_category_id_gl_category:Option<GlCategory>,
	/// has one
	pub c_doctype_id_c_doctype:Option<CDoctype>,
	/// has one
	pub c_acctschema_id_c_acctschema:Option<CAcctschema>,
}

pub struct ADepreciationExp {
	/// primary
	/// not nullable 
	pub a_depreciation_exp_id:f64,
	/// not nullable 
	pub a_account_number:String,
	/// not nullable 
	pub a_asset_id:f64,
	/// not nullable 
	pub a_entry_type:String,
	/// not nullable 
	pub a_period:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub dateacct:Option<NaiveDateTime>,
	/// not nullable 
	pub description:String,
	/// not nullable 
	pub expense:f64,
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub isdepreciated:String,
	pub postingtype:Option<String>,
	/// not nullable 
	pub processed:String,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
}

pub struct ADepreciationForecast {
	/// primary
	/// not nullable 
	pub a_depreciation_forecast_id:f64,
	/// not nullable 
	pub a_end_asset_id:f64,
	/// not nullable 
	pub a_start_asset_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub datedoc:NaiveDateTime,
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub postingtype:String,
	pub processed:Option<String>,
	pub processing:Option<String>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub a_end_asset_id_a_asset:Option<AAsset>,
	/// has one
	pub a_start_asset_id_a_asset:Option<AAsset>,
}

pub struct ADepreciationMethod {
	/// primary
	/// not nullable 
	pub a_depreciation_method_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub depreciationtype:Option<String>,
	pub description:Option<String>,
	/// not nullable 
	pub isactive:String,
	pub name:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub processed:String,
	pub text:Option<String>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has many
	pub a_asset_acct:Option<Vec<AAssetAcct>>,
}

pub struct ADepreciationTableDetail {
	/// primary
	/// not nullable 
	pub a_depreciation_table_detail_id:f64,
	/// not nullable 
	pub a_depreciation_rate:f64,
	/// not nullable 
	pub a_depreciation_table_code:String,
	/// not nullable 
	pub a_period:f64,
	/// defaults to: 'RT'::character varying
	pub a_table_rate_type:Option<String>,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub processed:String,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
}

pub struct ADepreciationTableHeader {
	/// primary
	/// not nullable 
	pub a_depreciation_table_header_id:f64,
	/// not nullable 
	pub a_depreciation_table_code:String,
	/// not nullable 
	pub a_table_rate_type:String,
	/// not nullable 
	pub a_term:String,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub description:String,
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub processed:String,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has many
	pub a_asset_acct:Option<Vec<AAssetAcct>>,
	/// has many
	pub a_asset_change:Option<Vec<AAssetChange>>,
	/// has many
	pub a_asset_group_acct:Option<Vec<AAssetGroupAcct>>,
	/// has many
	pub i_asset:Option<Vec<IAsset>>,
}

pub struct ADepreciationWorkfile {
	/// primary
	/// not nullable 
	pub a_depreciation_workfile_id:f64,
	/// defaults to: (0)::numeric
	pub a_accumulated_depr:Option<f64>,
	pub a_asset_cost:Option<f64>,
	/// not nullable 
	pub a_asset_id:f64,
	pub a_asset_life_current_year:Option<f64>,
	/// not nullable 
	pub a_asset_life_years:f64,
	pub a_base_amount:Option<f64>,
	/// defaults to: (0)::numeric
	pub a_calc_accumulated_depr:Option<f64>,
	pub a_curr_dep_exp:Option<f64>,
	pub a_current_period:Option<f64>,
	pub a_life_period:Option<f64>,
	pub a_period_forecast:Option<f64>,
	pub a_period_posted:Option<f64>,
	pub a_prior_year_accumulated_depr:Option<f64>,
	/// not nullable 
	pub a_qty_current:f64,
	pub a_salvage_value:Option<f64>,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub assetdepreciationdate:Option<NaiveDateTime>,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub dateacct:Option<NaiveDateTime>,
	/// not nullable 
	pub isactive:String,
	pub isdepreciated:Option<String>,
	pub postingtype:Option<String>,
	/// defaults to: 'Y'::bpchar
	pub processing:Option<String>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
}

pub struct ARegistration {
	/// primary
	/// not nullable 
	pub a_registration_id:f64,
	pub a_asset_id:Option<f64>,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_user_id:Option<f64>,
	pub assetservicedate:Option<NaiveDateTime>,
	pub c_bpartner_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isallowpublish:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isinproduction:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isregistered:String,
	pub m_product_id:Option<f64>,
	/// not nullable 
	pub name:String,
	pub note:Option<String>,
	pub processing:Option<String>,
	pub remote_addr:Option<String>,
	pub remote_host:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub a_asset_id_a_asset:Option<AAsset>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
	/// has many
	pub a_registrationvalue:Option<Vec<ARegistrationvalue>>,
}

pub struct ARegistrationattribute {
	/// primary
	/// not nullable 
	pub a_registrationattribute_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_reference_id:f64,
	pub ad_reference_value_id:Option<f64>,
	pub columnname:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isselfservice:String,
	/// not nullable 
	pub name:String,
	/// defaults to: 0
	/// not nullable 
	pub seqno:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_reference_id_ad_reference:Option<AdReference>,
	/// has one
	pub ad_reference_value_id_ad_reference:Option<AdReference>,
	/// has many
	pub a_registrationproduct:Option<Vec<ARegistrationproduct>>,
	/// has many
	pub a_registrationvalue:Option<Vec<ARegistrationvalue>>,
}

pub struct ARegistrationproduct {
	/// primary
	/// not nullable 
	pub a_registrationattribute_id:f64,
	/// primary
	/// not nullable 
	pub m_product_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub a_registrationattribute_id_a_registrationattribute:Option<ARegistrationattribute>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
}

pub struct ARegistrationvalue {
	/// primary
	/// not nullable 
	pub a_registration_id:f64,
	/// primary
	/// not nullable 
	pub a_registrationattribute_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub a_registration_id_a_registration:Option<ARegistration>,
	/// has one
	pub a_registrationattribute_id_a_registrationattribute:Option<ARegistrationattribute>,
}

pub struct AdAccesslog {
	/// primary
	/// not nullable 
	pub ad_accesslog_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	pub ad_column_id:Option<f64>,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_table_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub record_id:Option<f64>,
	pub remote_addr:Option<String>,
	pub remote_host:Option<String>,
	pub reply:Option<String>,
	pub textmsg:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_table_id_ad_table:Option<AdTable>,
	/// has one
	pub ad_column_id_ad_column:Option<AdColumn>,
}

pub struct AdAlert {
	/// primary
	/// not nullable 
	pub ad_alert_id:f64,
	pub ad_alertprocessor_id:Option<f64>,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub alertmessage:String,
	/// not nullable 
	pub alertsubject:String,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub enforceclientsecurity:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub enforcerolesecurity:String,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isvalid:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_alertprocessor_id_ad_alertprocessor:Option<AdAlertprocessor>,
	/// has many
	pub ad_alertrecipient:Option<Vec<AdAlertrecipient>>,
	/// has many
	pub ad_alertrule:Option<Vec<AdAlertrule>>,
}

pub struct AdAlertprocessor {
	/// primary
	/// not nullable 
	pub ad_alertprocessor_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub datelastrun:Option<NaiveDateTime>,
	pub datenextrun:Option<NaiveDateTime>,
	pub description:Option<String>,
	/// not nullable 
	pub frequency:f64,
	/// not nullable 
	pub frequencytype:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub keeplogdays:f64,
	/// not nullable 
	pub name:String,
	pub processing:Option<String>,
	/// not nullable 
	pub supervisor_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub supervisor_id_ad_user:Option<AdUser>,
	/// has many
	pub ad_alert:Option<Vec<AdAlert>>,
	/// has many
	pub ad_alertprocessorlog:Option<Vec<AdAlertprocessorlog>>,
}

pub struct AdAlertprocessorlog {
	/// primary
	/// not nullable 
	pub ad_alertprocessor_id:f64,
	/// primary
	/// not nullable 
	pub ad_alertprocessorlog_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub binarydata:Option<Vec<u8>>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub iserror:String,
	pub reference:Option<String>,
	pub summary:Option<String>,
	pub textmsg:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_alertprocessor_id_ad_alertprocessor:Option<AdAlertprocessor>,
}

pub struct AdAlertrecipient {
	/// primary
	/// not nullable 
	pub ad_alertrecipient_id:f64,
	/// not nullable 
	pub ad_alert_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_role_id:Option<f64>,
	pub ad_user_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_alert_id_ad_alert:Option<AdAlert>,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
	/// has one
	pub ad_role_id_ad_role:Option<AdRole>,
}

pub struct AdAlertrule {
	/// primary
	/// not nullable 
	pub ad_alertrule_id:f64,
	/// not nullable 
	pub ad_alert_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_table_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub errormsg:Option<String>,
	/// not nullable 
	pub fromclause:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isvalid:String,
	/// not nullable 
	pub name:String,
	pub otherclause:Option<String>,
	pub postprocessing:Option<String>,
	pub preprocessing:Option<String>,
	/// not nullable 
	pub selectclause:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub whereclause:Option<String>,
	/// has one
	pub ad_alert_id_ad_alert:Option<AdAlert>,
	/// has one
	pub ad_table_id_ad_table:Option<AdTable>,
}

pub struct AdArchive {
	/// primary
	/// not nullable 
	pub ad_archive_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_process_id:Option<f64>,
	pub ad_table_id:Option<f64>,
	pub binarydata:Option<Vec<u8>>,
	pub c_bpartner_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isreport:String,
	/// not nullable 
	pub name:String,
	pub record_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_table_id_ad_table:Option<AdTable>,
	/// has one
	pub ad_process_id_ad_process:Option<AdProcess>,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
}

pub struct AdAttachment {
	/// primary
	/// not nullable 
	pub ad_attachment_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_table_id:f64,
	pub binarydata:Option<Vec<u8>>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub record_id:f64,
	pub textmsg:Option<String>,
	/// not nullable 
	pub title:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_table_id_ad_table:Option<AdTable>,
	/// has many
	pub ad_attachmentnote:Option<Vec<AdAttachmentnote>>,
}

pub struct AdAttachmentnote {
	/// primary
	/// not nullable 
	pub ad_attachmentnote_id:f64,
	/// not nullable 
	pub ad_attachment_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_user_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub textmsg:String,
	/// not nullable 
	pub title:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_attachment_id_ad_attachment:Option<AdAttachment>,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
}

pub struct AdAttribute {
	/// primary
	/// not nullable 
	pub ad_attribute_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_reference_id:f64,
	pub ad_reference_value_id:Option<f64>,
	/// not nullable 
	pub ad_table_id:f64,
	pub ad_val_rule_id:Option<f64>,
	pub callout:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub defaultvalue:Option<String>,
	pub description:Option<String>,
	pub displaylength:Option<f64>,
	pub displaylogic:Option<String>,
	pub fieldlength:Option<f64>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isencrypted:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isfieldonly:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isheading:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub ismandatory:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isreadonly:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub issameline:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isupdateable:String,
	/// not nullable 
	pub name:String,
	pub seqno:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub valuemax:Option<String>,
	pub valuemin:Option<String>,
	pub vformat:Option<String>,
	/// has one
	pub ad_table_id_ad_table:Option<AdTable>,
	/// has one
	pub ad_reference_id_ad_reference:Option<AdReference>,
	/// has one
	pub ad_reference_value_id_ad_reference:Option<AdReference>,
	/// has one
	pub ad_val_rule_id_ad_val_rule:Option<AdValRule>,
	/// has many
	pub ad_attribute_value:Option<Vec<AdAttributeValue>>,
}

pub struct AdAttributeValue {
	/// primary
	/// not nullable 
	pub ad_attribute_id:f64,
	/// primary
	/// not nullable 
	pub record_id:f64,
	pub v_date:Option<NaiveDateTime>,
	pub v_number:Option<f64>,
	pub v_string:Option<String>,
	/// has one
	pub ad_attribute_id_ad_attribute:Option<AdAttribute>,
}

pub struct AdChangelog {
	/// primary
	/// not nullable 
	pub ad_changelog_id:f64,
	/// primary
	/// not nullable 
	pub ad_session_id:f64,
	/// primary
	/// not nullable 
	pub ad_table_id:f64,
	/// primary
	/// not nullable 
	pub ad_column_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub eventchangelog:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub iscustomization:String,
	pub newvalue:Option<String>,
	pub oldvalue:Option<String>,
	/// not nullable 
	pub record_id:f64,
	pub redo:Option<String>,
	pub trxname:Option<String>,
	pub undo:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_session_id_ad_session:Option<AdSession>,
	/// has one
	pub ad_table_id_ad_table:Option<AdTable>,
	/// has one
	pub ad_column_id_ad_column:Option<AdColumn>,
}

pub struct AdClass {
	/// primary
	/// not nullable 
	pub ad_class_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: NULL::numeric
	pub classpackage_id:Option<f64>,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: NULL::character varying
	pub description:Option<String>,
	/// defaults to: NULL::character varying
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// defaults to: NULL::character varying
	pub packagename:Option<String>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
}

pub struct AdClient {
	/// primary
	/// not nullable 
	pub ad_client_id:f64,
	pub ad_language:Option<String>,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_replicationstrategy_id:Option<f64>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub autoarchive:String,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub documentdir:Option<String>,
	pub emailtest:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub iscostimmediate:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub ismultilingualdocument:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub ispostimmediate:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isserveremail:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub issmtpauthorization:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isuseasp:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isusebetafunctions:String,
	pub ldapquery:Option<String>,
	/// defaults to: 'F'::bpchar
	/// not nullable 
	pub mmpolicy:String,
	pub modelvalidationclasses:Option<String>,
	/// not nullable 
	pub name:String,
	pub requestemail:Option<String>,
	pub requestfolder:Option<String>,
	pub requestuser:Option<String>,
	pub requestuserpw:Option<String>,
	pub smtphost:Option<String>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub storearchiveonfilesystem:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub storeattachmentsonfilesystem:String,
	pub unixarchivepath:Option<String>,
	pub unixattachmentpath:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
	pub windowsarchivepath:Option<String>,
	pub windowsattachmentpath:Option<String>,
	/// has one
	pub ad_language_ad_language:Option<AdLanguage>,
	/// has one
	pub ad_replicationstrategy_id_ad_replicationstrategy:Option<AdReplicationstrategy>,
	/// has many
	pub ad_clientinfo:Option<Vec<AdClientinfo>>,
	/// has many
	pub ad_clientshare:Option<Vec<AdClientshare>>,
	/// has many
	pub ad_column:Option<Vec<AdColumn>>,
	/// has many
	pub ad_field:Option<Vec<AdField>>,
	/// has many
	pub ad_language:Option<Vec<AdLanguage>>,
	/// has many
	pub ad_menu:Option<Vec<AdMenu>>,
	/// has many
	pub ad_message:Option<Vec<AdMessage>>,
	/// has many
	pub ad_org:Option<Vec<AdOrg>>,
	/// has many
	pub ad_preference:Option<Vec<AdPreference>>,
	/// has many
	pub ad_printform:Option<Vec<AdPrintform>>,
	/// has many
	pub ad_process_access:Option<Vec<AdProcessAccess>>,
	/// has many
	pub ad_ref_list:Option<Vec<AdRefList>>,
	/// has many
	pub ad_ref_table:Option<Vec<AdRefTable>>,
	/// has many
	pub ad_reference:Option<Vec<AdReference>>,
	/// has many
	pub ad_replication:Option<Vec<AdReplication>>,
	/// has many
	pub ad_role:Option<Vec<AdRole>>,
	/// has many
	pub ad_sequence:Option<Vec<AdSequence>>,
	/// has many
	pub ad_sequence_audit:Option<Vec<AdSequenceAudit>>,
	/// has many
	pub ad_sequence_no:Option<Vec<AdSequenceNo>>,
	/// has many
	pub ad_tab:Option<Vec<AdTab>>,
	/// has many
	pub ad_table:Option<Vec<AdTable>>,
	/// has many
	pub ad_table_access:Option<Vec<AdTableAccess>>,
	/// has many
	pub ad_task:Option<Vec<AdTask>>,
	/// has many
	pub ad_task_access:Option<Vec<AdTaskAccess>>,
	/// has many
	pub ad_taskinstance:Option<Vec<AdTaskinstance>>,
	/// has many
	pub ad_user:Option<Vec<AdUser>>,
	/// has many
	pub ad_user_roles:Option<Vec<AdUserRoles>>,
	/// has many
	pub ad_val_rule:Option<Vec<AdValRule>>,
	/// has many
	pub ad_wf_node:Option<Vec<AdWfNode>>,
	/// has many
	pub ad_wf_nodenext:Option<Vec<AdWfNodenext>>,
	/// has many
	pub ad_wf_process:Option<Vec<AdWfProcess>>,
	/// has many
	pub ad_window:Option<Vec<AdWindow>>,
	/// has many
	pub ad_window_access:Option<Vec<AdWindowAccess>>,
	/// has many
	pub ad_workflow:Option<Vec<AdWorkflow>>,
	/// has many
	pub ad_workflow_access:Option<Vec<AdWorkflowAccess>>,
	/// has many
	pub c_acctschema:Option<Vec<CAcctschema>>,
	/// has many
	pub c_acctschema_element:Option<Vec<CAcctschemaElement>>,
	/// has many
	pub c_bpartner:Option<Vec<CBpartner>>,
	/// has many
	pub c_bpartner_location:Option<Vec<CBpartnerLocation>>,
	/// has many
	pub c_calendar:Option<Vec<CCalendar>>,
	/// has many
	pub c_city:Option<Vec<CCity>>,
	/// has many
	pub c_conversion_rate:Option<Vec<CConversionRate>>,
	/// has many
	pub c_country:Option<Vec<CCountry>>,
	/// has many
	pub c_currency:Option<Vec<CCurrency>>,
	/// has many
	pub c_element:Option<Vec<CElement>>,
	/// has many
	pub c_elementvalue:Option<Vec<CElementvalue>>,
	/// has many
	pub c_location:Option<Vec<CLocation>>,
	/// has many
	pub c_nonbusinessday:Option<Vec<CNonbusinessday>>,
	/// has many
	pub c_period:Option<Vec<CPeriod>>,
	/// has many
	pub c_project:Option<Vec<CProject>>,
	/// has many
	pub c_region:Option<Vec<CRegion>>,
	/// has many
	pub c_uom:Option<Vec<CUom>>,
	/// has many
	pub c_uom_conversion:Option<Vec<CUomConversion>>,
	/// has many
	pub c_validcombination:Option<Vec<CValidcombination>>,
	/// has many
	pub c_year:Option<Vec<CYear>>,
	/// has many
	pub fact_acct:Option<Vec<FactAcct>>,
	/// has many
	pub gl_journalline:Option<Vec<GlJournalline>>,
	/// has many
	pub m_cost:Option<Vec<MCost>>,
	/// has many
	pub m_costdetail:Option<Vec<MCostdetail>>,
	/// has many
	pub m_locator:Option<Vec<MLocator>>,
	/// has many
	pub m_product:Option<Vec<MProduct>>,
	/// has many
	pub m_product_po:Option<Vec<MProductPo>>,
	/// has many
	pub m_storage:Option<Vec<MStorage>>,
	/// has many
	pub m_substitute:Option<Vec<MSubstitute>>,
	/// has many
	pub m_warehouse:Option<Vec<MWarehouse>>,
	/// has many
	pub w_store:Option<Vec<WStore>>,
}

pub struct AdClientinfo {
	/// primary
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_tree_activity_id:Option<f64>,
	pub ad_tree_bpartner_id:Option<f64>,
	pub ad_tree_campaign_id:Option<f64>,
	pub ad_tree_menu_id:Option<f64>,
	pub ad_tree_org_id:Option<f64>,
	pub ad_tree_product_id:Option<f64>,
	pub ad_tree_project_id:Option<f64>,
	pub ad_tree_salesregion_id:Option<f64>,
	pub c_acctschema1_id:Option<f64>,
	pub c_bpartnercashtrx_id:Option<f64>,
	pub c_calendar_id:Option<f64>,
	pub c_uom_length_id:Option<f64>,
	pub c_uom_time_id:Option<f64>,
	pub c_uom_volume_id:Option<f64>,
	pub c_uom_weight_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isdiscountlineamt:String,
	pub keeplogdays:Option<f64>,
	/// defaults to: NULL::numeric
	pub logo_id:Option<f64>,
	/// defaults to: NULL::numeric
	pub logoreport_id:Option<f64>,
	/// defaults to: NULL::numeric
	pub logoweb_id:Option<f64>,
	pub m_productfreight_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_client_id_ad_client:Option<AdClient>,
	/// has one
	pub c_calendar_id_c_calendar:Option<CCalendar>,
	/// has one
	pub c_acctschema1_id_c_acctschema:Option<CAcctschema>,
	/// has one
	pub c_uom_volume_id_c_uom:Option<CUom>,
	/// has one
	pub c_uom_weight_id_c_uom:Option<CUom>,
	/// has one
	pub c_uom_length_id_c_uom:Option<CUom>,
	/// has one
	pub c_uom_time_id_c_uom:Option<CUom>,
	/// has one
	pub ad_tree_menu_id_ad_tree:Option<AdTree>,
	/// has one
	pub ad_tree_org_id_ad_tree:Option<AdTree>,
	/// has one
	pub ad_tree_bpartner_id_ad_tree:Option<AdTree>,
	/// has one
	pub ad_tree_project_id_ad_tree:Option<AdTree>,
	/// has one
	pub ad_tree_salesregion_id_ad_tree:Option<AdTree>,
	/// has one
	pub ad_tree_product_id_ad_tree:Option<AdTree>,
	/// has one
	pub m_productfreight_id_m_product:Option<MProduct>,
	/// has one
	pub c_bpartnercashtrx_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub ad_tree_activity_id_ad_tree:Option<AdTree>,
	/// has one
	pub ad_tree_campaign_id_ad_tree:Option<AdTree>,
}

pub struct AdClientshare {
	/// primary
	/// not nullable 
	pub ad_clientshare_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_table_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub sharetype:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_client_id_ad_client:Option<AdClient>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub ad_table_id_ad_table:Option<AdTable>,
}

pub struct AdColor {
	/// primary
	/// not nullable 
	pub ad_color_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	pub ad_image_id:Option<f64>,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub alpha:f64,
	pub alpha_1:Option<f64>,
	/// not nullable 
	pub blue:f64,
	pub blue_1:Option<f64>,
	/// not nullable 
	pub colortype:String,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub green:f64,
	pub green_1:Option<f64>,
	/// not nullable 
	pub imagealpha:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isdefault:String,
	pub linedistance:Option<f64>,
	pub linewidth:Option<f64>,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub red:f64,
	pub red_1:Option<f64>,
	pub repeatdistance:Option<f64>,
	pub startpoint:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_image_id_ad_image:Option<AdImage>,
	/// has many
	pub ad_desktop:Option<Vec<AdDesktop>>,
	/// has many
	pub ad_window:Option<Vec<AdWindow>>,
	/// has many
	pub ad_workbench:Option<Vec<AdWorkbench>>,
}

pub struct AdColumn {
	/// primary
	/// not nullable 
	pub ad_column_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	pub ad_element_id:Option<f64>,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_process_id:Option<f64>,
	/// not nullable 
	pub ad_reference_id:f64,
	pub ad_reference_value_id:Option<f64>,
	/// not nullable 
	pub ad_table_id:f64,
	pub ad_val_rule_id:Option<f64>,
	/// defaults to: NULL::numeric
	pub attribute_id:Option<f64>,
	pub callout:Option<String>,
	/// not nullable 
	pub columnname:String,
	pub columnsql:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub defaultvalue:Option<String>,
	pub description:Option<String>,
	/// defaults to: 'D'::character varying
	/// not nullable 
	pub entitytype:String,
	pub fieldlength:Option<f64>,
	pub formatpattern:Option<String>,
	pub help:Option<String>,
	pub infofactoryclass:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	pub isallowlogging:Option<String>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isalwaysupdateable:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isautocomplete:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isencrypted:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isidentifier:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub iskey:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub ismandatory:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isparent:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isselectioncolumn:String,
	/// defaults to: 'N'::bpchar
	pub issyncdatabase:Option<String>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub istranslated:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isupdateable:String,
	pub mandatorylogic:Option<String>,
	/// not nullable 
	pub name:String,
	pub readonlylogic:Option<String>,
	pub seqno:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub valuemax:Option<String>,
	pub valuemin:Option<String>,
	/// not nullable 
	pub version:f64,
	pub vformat:Option<String>,
	/// has one
	pub ad_client_id_ad_client:Option<AdClient>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub entitytype_ad_entitytype:Option<AdEntitytype>,
	/// has one
	pub ad_table_id_ad_table:Option<AdTable>,
	/// has one
	pub ad_reference_id_ad_reference:Option<AdReference>,
	/// has one
	pub ad_reference_value_id_ad_reference:Option<AdReference>,
	/// has one
	pub ad_val_rule_id_ad_val_rule:Option<AdValRule>,
	/// has one
	pub ad_element_id_ad_element:Option<AdElement>,
	/// has one
	pub ad_process_id_ad_process:Option<AdProcess>,
	/// has many
	pub ad_accesslog:Option<Vec<AdAccesslog>>,
	/// has many
	pub ad_changelog:Option<Vec<AdChangelog>>,
	/// has many
	pub ad_column_access:Option<Vec<AdColumnAccess>>,
	/// has many
	pub ad_column_trl:Option<Vec<AdColumnTrl>>,
	/// has many
	pub ad_field:Option<Vec<AdField>>,
	/// has many
	pub ad_find:Option<Vec<AdFind>>,
	/// has many
	pub ad_impformat_row:Option<Vec<AdImpformatRow>>,
	/// has many
	pub ad_package_imp_backup:Option<Vec<AdPackageImpBackup>>,
	/// has many
	pub ad_printformatitem:Option<Vec<AdPrintformatitem>>,
	/// has many
	pub ad_printlabelline:Option<Vec<AdPrintlabelline>>,
	/// has many
	pub ad_ref_table:Option<Vec<AdRefTable>>,
	/// has many
	pub ad_reportview_col:Option<Vec<AdReportviewCol>>,
	/// has many
	pub ad_tab:Option<Vec<AdTab>>,
	/// has many
	pub ad_wf_nextcondition:Option<Vec<AdWfNextcondition>>,
	/// has many
	pub ad_wf_node:Option<Vec<AdWfNode>>,
	/// has many
	pub ad_workbench:Option<Vec<AdWorkbench>>,
	/// has many
	pub c_acctschema_element:Option<Vec<CAcctschemaElement>>,
	/// has many
	pub exp_formatline:Option<Vec<ExpFormatline>>,
	/// has many
	pub i_elementvalue:Option<Vec<IElementvalue>>,
	/// has many
	pub pp_order_node:Option<Vec<PpOrderNode>>,
}

pub struct AdColumnAccess {
	/// primary
	/// not nullable 
	pub ad_role_id:f64,
	/// primary
	/// not nullable 
	pub ad_column_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_table_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isexclude:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isreadonly:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_role_id_ad_role:Option<AdRole>,
	/// has one
	pub ad_column_id_ad_column:Option<AdColumn>,
	/// has one
	pub ad_table_id_ad_table:Option<AdTable>,
}

pub struct AdColumnTrl {
	/// primary
	/// not nullable 
	pub ad_column_id:f64,
	/// primary
	/// not nullable 
	pub ad_language:String,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub istranslated:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_column_id_ad_column:Option<AdColumn>,
	/// has one
	pub ad_language_ad_language:Option<AdLanguage>,
}

pub struct AdDesktop {
	/// primary
	/// not nullable 
	pub ad_desktop_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	pub ad_color_id:Option<f64>,
	pub ad_image_id:Option<f64>,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_image_id_ad_image:Option<AdImage>,
	/// has one
	pub ad_color_id_ad_color:Option<AdColor>,
	/// has many
	pub ad_desktop_trl:Option<Vec<AdDesktopTrl>>,
	/// has many
	pub ad_desktopworkbench:Option<Vec<AdDesktopworkbench>>,
}

pub struct AdDesktopTrl {
	/// primary
	/// not nullable 
	pub ad_desktop_id:f64,
	/// primary
	/// not nullable 
	pub ad_language:String,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub istranslated:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_desktop_id_ad_desktop:Option<AdDesktop>,
	/// has one
	pub ad_language_ad_language:Option<AdLanguage>,
}

pub struct AdDesktopworkbench {
	/// primary
	/// not nullable 
	pub ad_desktopworkbench_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_desktop_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_workbench_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub seqno:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_workbench_id_ad_workbench:Option<AdWorkbench>,
	/// has one
	pub ad_desktop_id_ad_desktop:Option<AdDesktop>,
}

pub struct AdDocumentActionAccess {
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_ref_list_id:f64,
	/// not nullable 
	pub ad_role_id:f64,
	/// not nullable 
	pub c_doctype_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDate,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDate,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_doctype_id_c_doctype:Option<CDoctype>,
	/// has one
	pub ad_role_id_ad_role:Option<AdRole>,
	/// has one
	pub ad_ref_list_id_ad_ref_list:Option<AdRefList>,
}

pub struct AdElement {
	/// primary
	/// not nullable 
	pub ad_element_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub columnname:String,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'D'::character varying
	/// not nullable 
	pub entitytype:String,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	pub po_description:Option<String>,
	pub po_help:Option<String>,
	pub po_name:Option<String>,
	pub po_printname:Option<String>,
	/// not nullable 
	pub printname:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub entitytype_ad_entitytype:Option<AdEntitytype>,
	/// has many
	pub ad_column:Option<Vec<AdColumn>>,
	/// has many
	pub ad_element_trl:Option<Vec<AdElementTrl>>,
	/// has many
	pub ad_infocolumn:Option<Vec<AdInfocolumn>>,
	/// has many
	pub ad_process_para:Option<Vec<AdProcessPara>>,
}

pub struct AdElementTrl {
	/// primary
	/// not nullable 
	pub ad_element_id:f64,
	/// primary
	/// not nullable 
	pub ad_language:String,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub istranslated:String,
	/// not nullable 
	pub name:String,
	pub po_description:Option<String>,
	pub po_help:Option<String>,
	pub po_name:Option<String>,
	pub po_printname:Option<String>,
	/// not nullable 
	pub printname:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_element_id_ad_element:Option<AdElement>,
	/// has one
	pub ad_language_ad_language:Option<AdLanguage>,
}

pub struct AdEntitytype {
	/// primary
	/// defaults to: 'D'::character varying
	/// not nullable 
	pub entitytype:String,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_entitytype_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub classpath:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub modelpackage:Option<String>,
	/// not nullable 
	pub name:String,
	pub processing:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub version:Option<String>,
	/// has many
	pub ad_column:Option<Vec<AdColumn>>,
	/// has many
	pub ad_element:Option<Vec<AdElement>>,
	/// has many
	pub ad_field:Option<Vec<AdField>>,
	/// has many
	pub ad_fieldgroup:Option<Vec<AdFieldgroup>>,
	/// has many
	pub ad_form:Option<Vec<AdForm>>,
	/// has many
	pub ad_image:Option<Vec<AdImage>>,
	/// has many
	pub ad_infocolumn:Option<Vec<AdInfocolumn>>,
	/// has many
	pub ad_infowindow:Option<Vec<AdInfowindow>>,
	/// has many
	pub ad_menu:Option<Vec<AdMenu>>,
	/// has many
	pub ad_message:Option<Vec<AdMessage>>,
	/// has many
	pub ad_modelvalidator:Option<Vec<AdModelvalidator>>,
	/// has many
	pub ad_modification:Option<Vec<AdModification>>,
	/// has many
	pub ad_process:Option<Vec<AdProcess>>,
	/// has many
	pub ad_process_para:Option<Vec<AdProcessPara>>,
	/// has many
	pub ad_ref_list:Option<Vec<AdRefList>>,
	/// has many
	pub ad_ref_table:Option<Vec<AdRefTable>>,
	/// has many
	pub ad_reference:Option<Vec<AdReference>>,
	/// has many
	pub ad_replicationstrategy:Option<Vec<AdReplicationstrategy>>,
	/// has many
	pub ad_replicationtable:Option<Vec<AdReplicationtable>>,
	/// has many
	pub ad_reportview:Option<Vec<AdReportview>>,
	/// has many
	pub ad_rule:Option<Vec<AdRule>>,
	/// has many
	pub ad_sysconfig:Option<Vec<AdSysconfig>>,
	/// has many
	pub ad_tab:Option<Vec<AdTab>>,
	/// has many
	pub ad_table:Option<Vec<AdTable>>,
	/// has many
	pub ad_task:Option<Vec<AdTask>>,
	/// has many
	pub ad_val_rule:Option<Vec<AdValRule>>,
	/// has many
	pub ad_wf_nextcondition:Option<Vec<AdWfNextcondition>>,
	/// has many
	pub ad_wf_node:Option<Vec<AdWfNode>>,
	/// has many
	pub ad_wf_node_para:Option<Vec<AdWfNodePara>>,
	/// has many
	pub ad_wf_nodenext:Option<Vec<AdWfNodenext>>,
	/// has many
	pub ad_wf_responsible:Option<Vec<AdWfResponsible>>,
	/// has many
	pub ad_window:Option<Vec<AdWindow>>,
	/// has many
	pub ad_workbench:Option<Vec<AdWorkbench>>,
	/// has many
	pub ad_workbenchwindow:Option<Vec<AdWorkbenchwindow>>,
	/// has many
	pub ad_workflow:Option<Vec<AdWorkflow>>,
	/// has many
	pub pa_colorschema:Option<Vec<PaColorschema>>,
	/// has many
	pub pa_measurecalc:Option<Vec<PaMeasurecalc>>,
	/// has many
	pub pp_order_node:Option<Vec<PpOrderNode>>,
	/// has many
	pub pp_order_nodenext:Option<Vec<PpOrderNodenext>>,
	/// has many
	pub pp_order_workflow:Option<Vec<PpOrderWorkflow>>,
	/// has many
	pub pp_wf_node_product:Option<Vec<PpWfNodeProduct>>,
}

pub struct AdError {
	/// primary
	/// not nullable 
	pub ad_error_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	pub ad_language:Option<String>,
	/// not nullable 
	pub ad_org_id:f64,
	pub code:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_language_ad_language:Option<AdLanguage>,
}

pub struct AdField {
	/// primary
	/// not nullable 
	pub ad_field_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	pub ad_column_id:Option<f64>,
	pub ad_fieldgroup_id:Option<f64>,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_reference_id:Option<f64>,
	pub ad_reference_value_id:Option<f64>,
	/// not nullable 
	pub ad_tab_id:f64,
	pub ad_val_rule_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub defaultvalue:Option<String>,
	pub description:Option<String>,
	pub displaylength:Option<f64>,
	pub displaylogic:Option<String>,
	/// defaults to: 'D'::character varying
	/// not nullable 
	pub entitytype:String,
	pub help:Option<String>,
	pub included_tab_id:Option<f64>,
	pub infofactoryclass:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub iscentrallymaintained:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isdisplayed:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isencrypted:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isfieldonly:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isheading:String,
	pub ismandatory:Option<String>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isreadonly:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub issameline:String,
	/// not nullable 
	pub name:String,
	pub obscuretype:Option<String>,
	pub seqno:Option<f64>,
	pub sortno:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_client_id_ad_client:Option<AdClient>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub ad_tab_id_ad_tab:Option<AdTab>,
	/// has one
	pub ad_column_id_ad_column:Option<AdColumn>,
	/// has one
	pub ad_fieldgroup_id_ad_fieldgroup:Option<AdFieldgroup>,
	/// has one
	pub entitytype_ad_entitytype:Option<AdEntitytype>,
	/// has one
	pub ad_reference_id_ad_reference:Option<AdReference>,
	/// has one
	pub ad_reference_value_id_ad_reference:Option<AdReference>,
	/// has one
	pub ad_val_rule_id_ad_val_rule:Option<AdValRule>,
	/// has many
	pub ad_field_trl:Option<Vec<AdFieldTrl>>,
	/// has many
	pub ad_userdef_field:Option<Vec<AdUserdefField>>,
	/// has many
	pub asp_clientexception:Option<Vec<AspClientexception>>,
	/// has many
	pub asp_field:Option<Vec<AspField>>,
}

pub struct AdFieldTrl {
	/// primary
	/// not nullable 
	pub ad_field_id:f64,
	/// primary
	/// not nullable 
	pub ad_language:String,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub istranslated:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_field_id_ad_field:Option<AdField>,
	/// has one
	pub ad_language_ad_language:Option<AdLanguage>,
}

pub struct AdFieldgroup {
	/// primary
	/// not nullable 
	pub ad_fieldgroup_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'D'::character varying
	/// not nullable 
	pub entitytype:String,
	pub fieldgrouptype:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	pub iscollapsedbydefault:Option<String>,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub entitytype_ad_entitytype:Option<AdEntitytype>,
	/// has many
	pub ad_field:Option<Vec<AdField>>,
	/// has many
	pub ad_fieldgroup_trl:Option<Vec<AdFieldgroupTrl>>,
}

pub struct AdFieldgroupTrl {
	/// primary
	/// not nullable 
	pub ad_fieldgroup_id:f64,
	/// primary
	/// not nullable 
	pub ad_language:String,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub istranslated:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_fieldgroup_id_ad_fieldgroup:Option<AdFieldgroup>,
	/// has one
	pub ad_language_ad_language:Option<AdLanguage>,
}

pub struct AdFind {
	/// primary
	/// not nullable 
	pub ad_find_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_column_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub andor:String,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub find_id:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub operation:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
	pub value2:Option<String>,
	/// has one
	pub ad_column_id_ad_column:Option<AdColumn>,
}

pub struct AdForm {
	/// primary
	/// not nullable 
	pub ad_form_id:f64,
	/// not nullable 
	pub accesslevel:String,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub classname:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'D'::character varying
	/// not nullable 
	pub entitytype:String,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isbetafunctionality:String,
	pub jspurl:Option<String>,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub entitytype_ad_entitytype:Option<AdEntitytype>,
	/// has many
	pub ad_form_access:Option<Vec<AdFormAccess>>,
	/// has many
	pub ad_form_trl:Option<Vec<AdFormTrl>>,
	/// has many
	pub ad_issue:Option<Vec<AdIssue>>,
	/// has many
	pub ad_menu:Option<Vec<AdMenu>>,
	/// has many
	pub ad_package_exp_common:Option<Vec<AdPackageExpCommon>>,
	/// has many
	pub ad_package_exp_detail:Option<Vec<AdPackageExpDetail>>,
	/// has many
	pub ad_wf_node:Option<Vec<AdWfNode>>,
	/// has many
	pub ad_workbenchwindow:Option<Vec<AdWorkbenchwindow>>,
	/// has many
	pub asp_clientexception:Option<Vec<AspClientexception>>,
	/// has many
	pub asp_form:Option<Vec<AspForm>>,
	/// has many
	pub pp_order_node:Option<Vec<PpOrderNode>>,
}

pub struct AdFormAccess {
	/// primary
	/// not nullable 
	pub ad_form_id:f64,
	/// primary
	/// not nullable 
	pub ad_role_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isreadwrite:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_form_id_ad_form:Option<AdForm>,
	/// has one
	pub ad_role_id_ad_role:Option<AdRole>,
}

pub struct AdFormTrl {
	/// primary
	/// not nullable 
	pub ad_form_id:f64,
	/// primary
	/// not nullable 
	pub ad_language:String,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub istranslated:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_form_id_ad_form:Option<AdForm>,
	/// has one
	pub ad_language_ad_language:Option<AdLanguage>,
}

pub struct AdHousekeeping {
	/// primary
	/// not nullable 
	pub ad_housekeeping_id:f64,
	/// defaults to: NULL::numeric
	/// not nullable 
	pub ad_client_id:f64,
	/// defaults to: NULL::numeric
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_table_id:f64,
	pub backupfolder:Option<String>,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub isexportxmlbackup:Option<String>,
	pub issaveinhistoric:Option<String>,
	pub lastdeleted:Option<f64>,
	pub lastrun:Option<NaiveDateTime>,
	/// not nullable 
	pub name:String,
	pub processing:Option<String>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
	pub whereclause:Option<String>,
}

pub struct AdImage {
	/// primary
	/// not nullable 
	pub ad_image_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub binarydata:Option<Vec<u8>>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'D'::character varying
	/// not nullable 
	pub entitytype:String,
	pub imageurl:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub entitytype_ad_entitytype:Option<AdEntitytype>,
	/// has many
	pub ad_color:Option<Vec<AdColor>>,
	/// has many
	pub ad_desktop:Option<Vec<AdDesktop>>,
	/// has many
	pub ad_tab:Option<Vec<AdTab>>,
	/// has many
	pub ad_wf_node:Option<Vec<AdWfNode>>,
	/// has many
	pub ad_window:Option<Vec<AdWindow>>,
	/// has many
	pub ad_workbench:Option<Vec<AdWorkbench>>,
	/// has many
	pub pp_order_node:Option<Vec<PpOrderNode>>,
}

pub struct AdImpformat {
	/// primary
	/// not nullable 
	pub ad_impformat_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_table_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// not nullable 
	pub formattype:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub processing:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_table_id_ad_table:Option<AdTable>,
	/// has many
	pub ad_impformat_row:Option<Vec<AdImpformatRow>>,
	/// has many
	pub ad_package_exp_common:Option<Vec<AdPackageExpCommon>>,
	/// has many
	pub ad_package_exp_detail:Option<Vec<AdPackageExpDetail>>,
}

pub struct AdImpformatRow {
	/// primary
	/// not nullable 
	pub ad_impformat_row_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_column_id:f64,
	/// not nullable 
	pub ad_impformat_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub callout:Option<String>,
	pub constantvalue:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub dataformat:Option<String>,
	/// not nullable 
	pub datatype:String,
	/// not nullable 
	pub decimalpoint:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub divideby100:String,
	pub endno:Option<f64>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	pub script:Option<String>,
	/// not nullable 
	pub seqno:f64,
	pub startno:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_impformat_id_ad_impformat:Option<AdImpformat>,
	/// has one
	pub ad_column_id_ad_column:Option<AdColumn>,
}

pub struct AdInfocolumn {
	/// primary
	/// not nullable 
	pub ad_infocolumn_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	pub ad_element_id:Option<f64>,
	/// not nullable 
	pub ad_infowindow_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_reference_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'D'::character varying
	/// not nullable 
	pub entitytype:String,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isdisplayed:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isquerycriteria:String,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub selectclause:String,
	/// defaults to: 0
	/// not nullable 
	pub seqno:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_infowindow_id_ad_infowindow:Option<AdInfowindow>,
	/// has one
	pub entitytype_ad_entitytype:Option<AdEntitytype>,
	/// has one
	pub ad_element_id_ad_element:Option<AdElement>,
	/// has one
	pub ad_reference_id_ad_reference:Option<AdReference>,
	/// has many
	pub ad_infocolumn_trl:Option<Vec<AdInfocolumnTrl>>,
}

pub struct AdInfocolumnTrl {
	/// primary
	/// not nullable 
	pub ad_infocolumn_id:f64,
	/// primary
	/// not nullable 
	pub ad_language:String,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub istranslated:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_infocolumn_id_ad_infocolumn:Option<AdInfocolumn>,
	/// has one
	pub ad_language_ad_language:Option<AdLanguage>,
}

pub struct AdInfowindow {
	/// primary
	/// not nullable 
	pub ad_infowindow_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_table_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'D'::character varying
	/// not nullable 
	pub entitytype:String,
	/// not nullable 
	pub fromclause:String,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	pub otherclause:Option<String>,
	pub processing:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_table_id_ad_table:Option<AdTable>,
	/// has one
	pub entitytype_ad_entitytype:Option<AdEntitytype>,
	/// has many
	pub ad_infocolumn:Option<Vec<AdInfocolumn>>,
	/// has many
	pub ad_infowindow_trl:Option<Vec<AdInfowindowTrl>>,
}

pub struct AdInfowindowTrl {
	/// primary
	/// not nullable 
	pub ad_infowindow_id:f64,
	/// primary
	/// not nullable 
	pub ad_language:String,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub istranslated:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_infowindow_id_ad_infowindow:Option<AdInfowindow>,
	/// has one
	pub ad_language_ad_language:Option<AdLanguage>,
}

pub struct AdIssue {
	/// primary
	/// not nullable 
	pub ad_issue_id:f64,
	pub a_asset_id:Option<f64>,
	/// not nullable 
	pub ad_client_id:f64,
	pub ad_form_id:Option<f64>,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_process_id:Option<f64>,
	pub ad_window_id:Option<f64>,
	pub comments:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub databaseinfo:Option<String>,
	pub dbaddress:Option<String>,
	pub errortrace:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	pub isreproducible:Option<String>,
	pub issuesource:Option<String>,
	/// not nullable 
	pub issuesummary:String,
	/// defaults to: 'N'::bpchar
	pub isvanillasystem:Option<String>,
	pub javainfo:Option<String>,
	/// defaults to: 0
	pub lineno:Option<f64>,
	pub local_host:Option<String>,
	pub loggername:Option<String>,
	/// not nullable 
	pub name:String,
	pub operatingsysteminfo:Option<String>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	pub processing:Option<String>,
	pub profileinfo:Option<String>,
	pub r_issueknown_id:Option<f64>,
	pub r_issueproject_id:Option<f64>,
	pub r_issuesystem_id:Option<f64>,
	pub r_issueuser_id:Option<f64>,
	pub r_request_id:Option<f64>,
	pub record_id:Option<f64>,
	/// not nullable 
	pub releaseno:String,
	pub releasetag:Option<String>,
	pub remote_addr:Option<String>,
	pub remote_host:Option<String>,
	pub requestdocumentno:Option<String>,
	pub responsetext:Option<String>,
	pub sourceclassname:Option<String>,
	pub sourcemethodname:Option<String>,
	pub stacktrace:Option<String>,
	pub statisticsinfo:Option<String>,
	pub supportemail:Option<String>,
	/// not nullable 
	pub systemstatus:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub username:String,
	/// not nullable 
	pub version:String,
	/// has one
	pub a_asset_id_a_asset:Option<AAsset>,
	/// has one
	pub r_request_id_r_request:Option<RRequest>,
	/// has one
	pub r_issueknown_id_r_issueknown:Option<RIssueknown>,
	/// has one
	pub r_issueproject_id_r_issueproject:Option<RIssueproject>,
	/// has one
	pub r_issueuser_id_r_issueuser:Option<RIssueuser>,
	/// has one
	pub r_issuesystem_id_r_issuesystem:Option<RIssuesystem>,
	/// has one
	pub ad_window_id_ad_window:Option<AdWindow>,
	/// has one
	pub ad_process_id_ad_process:Option<AdProcess>,
	/// has one
	pub ad_form_id_ad_form:Option<AdForm>,
}

pub struct AdLabelprinter {
	/// primary
	/// not nullable 
	pub ad_labelprinter_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has many
	pub ad_labelprinterfunction:Option<Vec<AdLabelprinterfunction>>,
	/// has many
	pub ad_printlabel:Option<Vec<AdPrintlabel>>,
}

pub struct AdLabelprinterfunction {
	/// primary
	/// not nullable 
	pub ad_labelprinterfunction_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_labelprinter_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub functionprefix:Option<String>,
	pub functionsuffix:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isxyposition:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub xyseparator:Option<String>,
	/// has one
	pub ad_labelprinter_id_ad_labelprinter:Option<AdLabelprinter>,
	/// has many
	pub ad_printlabelline:Option<Vec<AdPrintlabelline>>,
}

pub struct AdLanguage {
	/// primary
	/// not nullable 
	pub ad_language:String,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_language_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub countrycode:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub datepattern:Option<String>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isbaselanguage:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isdecimalpoint:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub issystemlanguage:String,
	pub languageiso:Option<String>,
	/// not nullable 
	pub name:String,
	pub processing:Option<String>,
	pub timepattern:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_client_id_ad_client:Option<AdClient>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has many
	pub ad_client:Option<Vec<AdClient>>,
	/// has many
	pub ad_column_trl:Option<Vec<AdColumnTrl>>,
	/// has many
	pub ad_desktop_trl:Option<Vec<AdDesktopTrl>>,
	/// has many
	pub ad_element_trl:Option<Vec<AdElementTrl>>,
	/// has many
	pub ad_error:Option<Vec<AdError>>,
	/// has many
	pub ad_field_trl:Option<Vec<AdFieldTrl>>,
	/// has many
	pub ad_fieldgroup_trl:Option<Vec<AdFieldgroupTrl>>,
	/// has many
	pub ad_form_trl:Option<Vec<AdFormTrl>>,
	/// has many
	pub ad_infocolumn_trl:Option<Vec<AdInfocolumnTrl>>,
	/// has many
	pub ad_infowindow_trl:Option<Vec<AdInfowindowTrl>>,
	/// has many
	pub ad_menu_trl:Option<Vec<AdMenuTrl>>,
	/// has many
	pub ad_message_trl:Option<Vec<AdMessageTrl>>,
	/// has many
	pub ad_printformatitem_trl:Option<Vec<AdPrintformatitemTrl>>,
	/// has many
	pub ad_printlabelline_trl:Option<Vec<AdPrintlabellineTrl>>,
	/// has many
	pub ad_process_para_trl:Option<Vec<AdProcessParaTrl>>,
	/// has many
	pub ad_process_trl:Option<Vec<AdProcessTrl>>,
	/// has many
	pub ad_ref_list_trl:Option<Vec<AdRefListTrl>>,
	/// has many
	pub ad_reference_trl:Option<Vec<AdReferenceTrl>>,
	/// has many
	pub ad_tab_trl:Option<Vec<AdTabTrl>>,
	/// has many
	pub ad_table_trl:Option<Vec<AdTableTrl>>,
	/// has many
	pub ad_task_trl:Option<Vec<AdTaskTrl>>,
	/// has many
	pub ad_userdef_win:Option<Vec<AdUserdefWin>>,
	/// has many
	pub ad_wf_node_trl:Option<Vec<AdWfNodeTrl>>,
	/// has many
	pub ad_window_trl:Option<Vec<AdWindowTrl>>,
	/// has many
	pub ad_workbench_trl:Option<Vec<AdWorkbenchTrl>>,
	/// has many
	pub ad_workflow_trl:Option<Vec<AdWorkflowTrl>>,
	/// has many
	pub c_bpartner:Option<Vec<CBpartner>>,
	/// has many
	pub c_country:Option<Vec<CCountry>>,
	/// has many
	pub c_country_trl:Option<Vec<CCountryTrl>>,
	/// has many
	pub c_currency_trl:Option<Vec<CCurrencyTrl>>,
	/// has many
	pub c_doctype_trl:Option<Vec<CDoctypeTrl>>,
	/// has many
	pub c_dunninglevel_trl:Option<Vec<CDunninglevelTrl>>,
	/// has many
	pub c_elementvalue_trl:Option<Vec<CElementvalueTrl>>,
	/// has many
	pub c_greeting_trl:Option<Vec<CGreetingTrl>>,
	/// has many
	pub c_paymentterm_trl:Option<Vec<CPaymenttermTrl>>,
	/// has many
	pub c_tax_trl:Option<Vec<CTaxTrl>>,
	/// has many
	pub c_taxcategory_trl:Option<Vec<CTaxcategoryTrl>>,
	/// has many
	pub c_uom_trl:Option<Vec<CUomTrl>>,
	/// has many
	pub cm_container_element_trl:Option<Vec<CmContainerElementTrl>>,
	/// has many
	pub cm_container_trl:Option<Vec<CmContainerTrl>>,
	/// has many
	pub cm_cstage_element_trl:Option<Vec<CmCstageElementTrl>>,
	/// has many
	pub cm_cstage_trl:Option<Vec<CmCstageTrl>>,
	/// has many
	pub cm_newschannel:Option<Vec<CmNewschannel>>,
	/// has many
	pub k_synonym:Option<Vec<KSynonym>>,
	/// has many
	pub m_product_trl:Option<Vec<MProductTrl>>,
	/// has many
	pub r_mailtext_trl:Option<Vec<RMailtextTrl>>,
	/// has many
	pub w_mailmsg_trl:Option<Vec<WMailmsgTrl>>,
	/// has many
	pub w_store_trl:Option<Vec<WStoreTrl>>,
}

pub struct AdLdapaccess {
	/// primary
	/// not nullable 
	pub ad_ldapaccess_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_ldapprocessor_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_user_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub iserror:String,
	pub r_interestarea_id:Option<f64>,
	pub summary:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_ldapprocessor_id_ad_ldapprocessor:Option<AdLdapprocessor>,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
	/// has one
	pub r_interestarea_id_r_interestarea:Option<RInterestarea>,
}

pub struct AdLdapprocessor {
	/// primary
	/// not nullable 
	pub ad_ldapprocessor_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub datelastrun:Option<NaiveDateTime>,
	pub datenextrun:Option<NaiveDateTime>,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 0
	/// not nullable 
	pub keeplogdays:f64,
	/// defaults to: 0
	/// not nullable 
	pub ldapport:f64,
	/// not nullable 
	pub name:String,
	pub processing:Option<String>,
	/// not nullable 
	pub supervisor_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub supervisor_id_ad_user:Option<AdUser>,
	/// has many
	pub ad_ldapaccess:Option<Vec<AdLdapaccess>>,
	/// has many
	pub ad_ldapprocessorlog:Option<Vec<AdLdapprocessorlog>>,
}

pub struct AdLdapprocessorlog {
	/// primary
	/// not nullable 
	pub ad_ldapprocessor_id:f64,
	/// primary
	/// not nullable 
	pub ad_ldapprocessorlog_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub binarydata:Option<Vec<u8>>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub iserror:String,
	pub reference:Option<String>,
	pub summary:Option<String>,
	pub textmsg:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_ldapprocessor_id_ad_ldapprocessor:Option<AdLdapprocessor>,
}

pub struct AdMenu {
	/// primary
	/// not nullable 
	pub ad_menu_id:f64,
	pub action:Option<String>,
	/// not nullable 
	pub ad_client_id:f64,
	pub ad_form_id:Option<f64>,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_process_id:Option<f64>,
	pub ad_task_id:Option<f64>,
	pub ad_window_id:Option<f64>,
	pub ad_workbench_id:Option<f64>,
	pub ad_workflow_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'D'::character varying
	/// not nullable 
	pub entitytype:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isreadonly:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub issotrx:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub issummary:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_client_id_ad_client:Option<AdClient>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub ad_window_id_ad_window:Option<AdWindow>,
	/// has one
	pub ad_workflow_id_ad_workflow:Option<AdWorkflow>,
	/// has one
	pub ad_task_id_ad_task:Option<AdTask>,
	/// has one
	pub ad_process_id_ad_process:Option<AdProcess>,
	/// has one
	pub ad_form_id_ad_form:Option<AdForm>,
	/// has one
	pub ad_workbench_id_ad_workbench:Option<AdWorkbench>,
	/// has one
	pub entitytype_ad_entitytype:Option<AdEntitytype>,
	/// has many
	pub ad_menu_trl:Option<Vec<AdMenuTrl>>,
	/// has many
	pub ad_package_exp_common:Option<Vec<AdPackageExpCommon>>,
	/// has many
	pub ad_package_exp_detail:Option<Vec<AdPackageExpDetail>>,
}

pub struct AdMenuTrl {
	/// primary
	/// not nullable 
	pub ad_menu_id:f64,
	/// primary
	/// not nullable 
	pub ad_language:String,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub istranslated:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_menu_id_ad_menu:Option<AdMenu>,
	/// has one
	pub ad_language_ad_language:Option<AdLanguage>,
}

pub struct AdMessage {
	/// primary
	/// not nullable 
	pub ad_message_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'D'::character varying
	/// not nullable 
	pub entitytype:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub msgtext:String,
	pub msgtip:Option<String>,
	/// not nullable 
	pub msgtype:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
	/// has one
	pub ad_client_id_ad_client:Option<AdClient>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub entitytype_ad_entitytype:Option<AdEntitytype>,
	/// has many
	pub ad_message_trl:Option<Vec<AdMessageTrl>>,
	/// has many
	pub ad_note:Option<Vec<AdNote>>,
	/// has many
	pub ad_package_exp_detail:Option<Vec<AdPackageExpDetail>>,
	/// has many
	pub ad_wf_activity:Option<Vec<AdWfActivity>>,
	/// has many
	pub ad_wf_process:Option<Vec<AdWfProcess>>,
}

pub struct AdMessageTrl {
	/// primary
	/// not nullable 
	pub ad_message_id:f64,
	/// primary
	/// not nullable 
	pub ad_language:String,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub istranslated:String,
	/// not nullable 
	pub msgtext:String,
	pub msgtip:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_message_id_ad_message:Option<AdMessage>,
	/// has one
	pub ad_language_ad_language:Option<AdLanguage>,
}

pub struct AdMigrationscript {
	/// primary
	/// not nullable 
	pub ad_migrationscript_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub developername:Option<String>,
	/// not nullable 
	pub filename:String,
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub isapply:String,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub projectname:String,
	pub reference:Option<String>,
	/// not nullable 
	pub releaseno:String,
	pub script:Option<Vec<u8>>,
	pub scriptroll:Option<String>,
	/// not nullable 
	pub status:String,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub url:Option<String>,
}

pub struct AdModelvalidator {
	/// primary
	/// not nullable 
	pub ad_modelvalidator_id:f64,
	/// defaults to: 0
	/// not nullable 
	pub ad_client_id:f64,
	/// defaults to: 0
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// not nullable 
	pub entitytype:String,
	pub help:Option<String>,
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub modelvalidationclass:String,
	/// not nullable 
	pub name:String,
	pub seqno:Option<f64>,
	/// not nullable 
	pub updated:NaiveDate,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub entitytype_ad_entitytype:Option<AdEntitytype>,
	/// has many
	pub ad_package_exp_detail:Option<Vec<AdPackageExpDetail>>,
}

pub struct AdModification {
	/// primary
	/// not nullable 
	pub ad_modification_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'D'::character varying
	/// not nullable 
	pub entitytype:String,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// defaults to: 0
	/// not nullable 
	pub seqno:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub version:Option<String>,
	/// has one
	pub entitytype_ad_entitytype:Option<AdEntitytype>,
}

pub struct AdNote {
	/// primary
	/// not nullable 
	pub ad_note_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_message_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_table_id:Option<f64>,
	pub ad_user_id:Option<f64>,
	pub ad_wf_activity_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	pub processed:Option<String>,
	pub processing:Option<String>,
	pub record_id:Option<f64>,
	pub reference:Option<String>,
	pub textmsg:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
	/// has one
	pub ad_message_id_ad_message:Option<AdMessage>,
	/// has one
	pub ad_table_id_ad_table:Option<AdTable>,
	/// has one
	pub ad_wf_activity_id_ad_wf_activity:Option<AdWfActivity>,
}

pub struct AdOrg {
	/// primary
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	pub ad_replicationstrategy_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub issummary:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
	/// has one
	pub ad_client_id_ad_client:Option<AdClient>,
	/// has one
	pub ad_replicationstrategy_id_ad_replicationstrategy:Option<AdReplicationstrategy>,
	/// has many
	pub ad_clientshare:Option<Vec<AdClientshare>>,
	/// has many
	pub ad_column:Option<Vec<AdColumn>>,
	/// has many
	pub ad_field:Option<Vec<AdField>>,
	/// has many
	pub ad_language:Option<Vec<AdLanguage>>,
	/// has many
	pub ad_menu:Option<Vec<AdMenu>>,
	/// has many
	pub ad_message:Option<Vec<AdMessage>>,
	/// has many
	pub ad_orginfo:Option<Vec<AdOrginfo>>,
	/// has many
	pub ad_preference:Option<Vec<AdPreference>>,
	/// has many
	pub ad_process_access:Option<Vec<AdProcessAccess>>,
	/// has many
	pub ad_ref_list:Option<Vec<AdRefList>>,
	/// has many
	pub ad_ref_table:Option<Vec<AdRefTable>>,
	/// has many
	pub ad_reference:Option<Vec<AdReference>>,
	/// has many
	pub ad_replication:Option<Vec<AdReplication>>,
	/// has many
	pub ad_role:Option<Vec<AdRole>>,
	/// has many
	pub ad_role_orgaccess:Option<Vec<AdRoleOrgaccess>>,
	/// has many
	pub ad_sequence:Option<Vec<AdSequence>>,
	/// has many
	pub ad_sequence_audit:Option<Vec<AdSequenceAudit>>,
	/// has many
	pub ad_sequence_no:Option<Vec<AdSequenceNo>>,
	/// has many
	pub ad_tab:Option<Vec<AdTab>>,
	/// has many
	pub ad_table:Option<Vec<AdTable>>,
	/// has many
	pub ad_table_access:Option<Vec<AdTableAccess>>,
	/// has many
	pub ad_task:Option<Vec<AdTask>>,
	/// has many
	pub ad_task_access:Option<Vec<AdTaskAccess>>,
	/// has many
	pub ad_taskinstance:Option<Vec<AdTaskinstance>>,
	/// has many
	pub ad_user:Option<Vec<AdUser>>,
	/// has many
	pub ad_user_orgaccess:Option<Vec<AdUserOrgaccess>>,
	/// has many
	pub ad_user_roles:Option<Vec<AdUserRoles>>,
	/// has many
	pub ad_val_rule:Option<Vec<AdValRule>>,
	/// has many
	pub ad_wf_node:Option<Vec<AdWfNode>>,
	/// has many
	pub ad_wf_nodenext:Option<Vec<AdWfNodenext>>,
	/// has many
	pub ad_wf_process:Option<Vec<AdWfProcess>>,
	/// has many
	pub ad_wf_responsible:Option<Vec<AdWfResponsible>>,
	/// has many
	pub ad_window:Option<Vec<AdWindow>>,
	/// has many
	pub ad_window_access:Option<Vec<AdWindowAccess>>,
	/// has many
	pub ad_workflow:Option<Vec<AdWorkflow>>,
	/// has many
	pub ad_workflow_access:Option<Vec<AdWorkflowAccess>>,
	/// has many
	pub c_acctschema:Option<Vec<CAcctschema>>,
	/// has many
	pub c_acctschema_element:Option<Vec<CAcctschemaElement>>,
	/// has many
	pub c_bpartner:Option<Vec<CBpartner>>,
	/// has many
	pub c_bpartner_location:Option<Vec<CBpartnerLocation>>,
	/// has many
	pub c_calendar:Option<Vec<CCalendar>>,
	/// has many
	pub c_cash:Option<Vec<CCash>>,
	/// has many
	pub c_city:Option<Vec<CCity>>,
	/// has many
	pub c_commissionline:Option<Vec<CCommissionline>>,
	/// has many
	pub c_conversion_rate:Option<Vec<CConversionRate>>,
	/// has many
	pub c_country:Option<Vec<CCountry>>,
	/// has many
	pub c_currency:Option<Vec<CCurrency>>,
	/// has many
	pub c_element:Option<Vec<CElement>>,
	/// has many
	pub c_elementvalue:Option<Vec<CElementvalue>>,
	/// has many
	pub c_interorg_acct:Option<Vec<CInterorgAcct>>,
	/// has many
	pub c_invoice:Option<Vec<CInvoice>>,
	/// has many
	pub c_invoicebatchline:Option<Vec<CInvoicebatchline>>,
	/// has many
	pub c_invoiceline:Option<Vec<CInvoiceline>>,
	/// has many
	pub c_location:Option<Vec<CLocation>>,
	/// has many
	pub c_nonbusinessday:Option<Vec<CNonbusinessday>>,
	/// has many
	pub c_order:Option<Vec<COrder>>,
	/// has many
	pub c_orderline:Option<Vec<COrderline>>,
	/// has many
	pub c_orgassignment:Option<Vec<COrgassignment>>,
	/// has many
	pub c_payment:Option<Vec<CPayment>>,
	/// has many
	pub c_period:Option<Vec<CPeriod>>,
	/// has many
	pub c_project:Option<Vec<CProject>>,
	/// has many
	pub c_region:Option<Vec<CRegion>>,
	/// has many
	pub c_uom:Option<Vec<CUom>>,
	/// has many
	pub c_uom_conversion:Option<Vec<CUomConversion>>,
	/// has many
	pub c_validcombination:Option<Vec<CValidcombination>>,
	/// has many
	pub c_year:Option<Vec<CYear>>,
	/// has many
	pub dd_order:Option<Vec<DdOrder>>,
	/// has many
	pub dd_orderline:Option<Vec<DdOrderline>>,
	/// has many
	pub fact_acct:Option<Vec<FactAcct>>,
	/// has many
	pub gl_distribution:Option<Vec<GlDistribution>>,
	/// has many
	pub gl_distributionline:Option<Vec<GlDistributionline>>,
	/// has many
	pub gl_journalline:Option<Vec<GlJournalline>>,
	/// has many
	pub hr_movement:Option<Vec<HrMovement>>,
	/// has many
	pub i_fajournal:Option<Vec<IFajournal>>,
	/// has many
	pub i_gljournal:Option<Vec<IGljournal>>,
	/// has many
	pub i_invoice:Option<Vec<IInvoice>>,
	/// has many
	pub i_order:Option<Vec<IOrder>>,
	/// has many
	pub m_cost:Option<Vec<MCost>>,
	/// has many
	pub m_costdetail:Option<Vec<MCostdetail>>,
	/// has many
	pub m_inout:Option<Vec<MInout>>,
	/// has many
	pub m_inoutline:Option<Vec<MInoutline>>,
	/// has many
	pub m_inventory:Option<Vec<MInventory>>,
	/// has many
	pub m_locator:Option<Vec<MLocator>>,
	/// has many
	pub m_movement:Option<Vec<MMovement>>,
	/// has many
	pub m_product:Option<Vec<MProduct>>,
	/// has many
	pub m_product_po:Option<Vec<MProductPo>>,
	/// has many
	pub m_production:Option<Vec<MProduction>>,
	/// has many
	pub m_storage:Option<Vec<MStorage>>,
	/// has many
	pub m_substitute:Option<Vec<MSubstitute>>,
	/// has many
	pub m_warehouse:Option<Vec<MWarehouse>>,
	/// has many
	pub pa_goalrestriction:Option<Vec<PaGoalrestriction>>,
	/// has many
	pub pa_report:Option<Vec<PaReport>>,
	/// has many
	pub pa_reportcolumn:Option<Vec<PaReportcolumn>>,
	/// has many
	pub pa_reportsource:Option<Vec<PaReportsource>>,
	/// has many
	pub pp_cost_collector:Option<Vec<PpCostCollector>>,
	/// has many
	pub pp_order:Option<Vec<PpOrder>>,
}

pub struct AdOrginfo {
	/// primary
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	pub ad_orgtype_id:Option<f64>,
	pub c_calendar_id:Option<f64>,
	pub c_location_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub dropship_warehouse_id:Option<f64>,
	/// not nullable 
	pub duns:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: NULL::numeric
	pub logo_id:Option<f64>,
	pub m_warehouse_id:Option<f64>,
	pub pa_goal_id:Option<f64>,
	pub parent_org_id:Option<f64>,
	pub receiptfootermsg:Option<String>,
	pub supervisor_id:Option<f64>,
	/// not nullable 
	pub taxid:String,
	pub transferbank_id:Option<f64>,
	pub transfercashbook_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub c_location_id_c_location:Option<CLocation>,
	/// has one
	pub supervisor_id_ad_user:Option<AdUser>,
	/// has one
	pub parent_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub ad_orgtype_id_ad_orgtype:Option<AdOrgtype>,
	/// has one
	pub m_warehouse_id_m_warehouse:Option<MWarehouse>,
	/// has one
	pub transferbank_id_c_bank:Option<CBank>,
	/// has one
	pub transfercashbook_id_c_cashbook:Option<CCashbook>,
}

pub struct AdOrgtype {
	/// primary
	/// not nullable 
	pub ad_orgtype_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_printcolor_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_printcolor_id_ad_printcolor:Option<AdPrintcolor>,
	/// has many
	pub ad_orginfo:Option<Vec<AdOrginfo>>,
	/// has many
	pub c_taxdefinition:Option<Vec<CTaxdefinition>>,
}

pub struct AdPackageExp {
	/// primary
	/// not nullable 
	pub ad_package_exp_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_package_type:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub description:String,
	/// not nullable 
	pub email:String,
	/// not nullable 
	pub file_directory:String,
	/// not nullable 
	pub instructions:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub pk_version:String,
	pub processed:Option<String>,
	/// not nullable 
	pub processing:String,
	/// not nullable 
	pub releaseno:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub username:String,
	/// not nullable 
	pub version:String,
}

pub struct AdPackageExpCommon {
	/// primary
	/// not nullable 
	pub ad_package_exp_common_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	pub ad_form_id:Option<f64>,
	pub ad_impformat_id:Option<f64>,
	pub ad_menu_id:Option<f64>,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_process_id:Option<f64>,
	pub ad_reportview_id:Option<f64>,
	pub ad_role_id:Option<f64>,
	pub ad_table_id:Option<f64>,
	pub ad_window_id:Option<f64>,
	pub ad_workbench_id:Option<f64>,
	pub ad_workflow_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub dbtype:Option<String>,
	pub description:Option<String>,
	pub destination_directory:Option<String>,
	pub file_directory:Option<String>,
	pub filename:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub line:Option<f64>,
	pub name:Option<String>,
	pub name2:Option<String>,
	pub processed:Option<String>,
	pub processing:Option<String>,
	pub sqlstatement:Option<String>,
	pub target_directory:Option<String>,
	pub type_:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_form_id_ad_form:Option<AdForm>,
	/// has one
	pub ad_impformat_id_ad_impformat:Option<AdImpformat>,
	/// has one
	pub ad_reportview_id_ad_reportview:Option<AdReportview>,
	/// has one
	pub ad_table_id_ad_table:Option<AdTable>,
	/// has one
	pub ad_workbench_id_ad_workbench:Option<AdWorkbench>,
	/// has one
	pub ad_workflow_id_ad_workflow:Option<AdWorkflow>,
	/// has one
	pub ad_window_id_ad_window:Option<AdWindow>,
	/// has one
	pub ad_role_id_ad_role:Option<AdRole>,
	/// has one
	pub ad_process_id_ad_process:Option<AdProcess>,
	/// has one
	pub ad_menu_id_ad_menu:Option<AdMenu>,
}

pub struct AdPackageExpDetail {
	/// primary
	/// not nullable 
	pub ad_package_exp_detail_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// defaults to: NULL::numeric
	pub ad_entitytype_id:Option<f64>,
	pub ad_form_id:Option<f64>,
	pub ad_impformat_id:Option<f64>,
	pub ad_menu_id:Option<f64>,
	pub ad_message_id:Option<f64>,
	/// defaults to: NULL::numeric
	pub ad_modelvalidator_id:Option<f64>,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_package_code_new:Option<String>,
	pub ad_package_code_old:Option<String>,
	/// not nullable 
	pub ad_package_exp_id:f64,
	pub ad_printformat_id:Option<f64>,
	pub ad_process_id:Option<f64>,
	pub ad_reference_id:Option<f64>,
	pub ad_reportview_id:Option<f64>,
	pub ad_role_id:Option<f64>,
	pub ad_table_id:Option<f64>,
	pub ad_val_rule_id:Option<f64>,
	pub ad_window_id:Option<f64>,
	pub ad_workbench_id:Option<f64>,
	pub ad_workflow_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub dbtype:Option<String>,
	/// not nullable 
	pub description:String,
	pub destination_directory:Option<String>,
	pub destination_filename:Option<String>,
	pub file_directory:Option<String>,
	pub filename:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub line:Option<f64>,
	pub name2:Option<String>,
	pub processed:Option<String>,
	/// not nullable 
	pub processing:String,
	pub releaseno:Option<String>,
	pub sqlstatement:Option<String>,
	pub target_directory:Option<String>,
	/// not nullable 
	pub type_:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_form_id_ad_form:Option<AdForm>,
	/// has one
	pub ad_impformat_id_ad_impformat:Option<AdImpformat>,
	/// has one
	pub ad_menu_id_ad_menu:Option<AdMenu>,
	/// has one
	pub ad_process_id_ad_process:Option<AdProcess>,
	/// has one
	pub ad_role_id_ad_role:Option<AdRole>,
	/// has one
	pub ad_window_id_ad_window:Option<AdWindow>,
	/// has one
	pub ad_workflow_id_ad_workflow:Option<AdWorkflow>,
	/// has one
	pub ad_workbench_id_ad_workbench:Option<AdWorkbench>,
	/// has one
	pub ad_table_id_ad_table:Option<AdTable>,
	/// has one
	pub ad_reportview_id_ad_reportview:Option<AdReportview>,
	/// has one
	pub ad_message_id_ad_message:Option<AdMessage>,
	/// has one
	pub ad_printformat_id_ad_printformat:Option<AdPrintformat>,
	/// has one
	pub ad_reference_id_ad_reference:Option<AdReference>,
	/// has one
	pub ad_modelvalidator_id_ad_modelvalidator:Option<AdModelvalidator>,
}

pub struct AdPackageImp {
	/// primary
	/// not nullable 
	pub ad_package_imp_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub createddate:Option<String>,
	pub creator:Option<String>,
	pub creatorcontact:Option<String>,
	/// not nullable 
	pub description:String,
	pub email:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	pub pk_status:Option<String>,
	pub pk_version:Option<String>,
	/// defaults to: 'N'::bpchar
	pub processed:Option<String>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processing:String,
	pub releaseno:Option<String>,
	pub uninstall:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub updateddate:Option<String>,
	pub version:Option<String>,
}

pub struct AdPackageImpBackup {
	/// primary
	/// not nullable 
	pub ad_package_imp_backup_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	pub ad_column_id:Option<f64>,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_package_imp_bck_dir:Option<String>,
	/// not nullable 
	pub ad_package_imp_detail_id:f64,
	/// not nullable 
	pub ad_package_imp_id:f64,
	pub ad_package_imp_org_dir:Option<String>,
	pub ad_reference_id:Option<f64>,
	pub ad_table_id:Option<f64>,
	pub colvalue:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub uninstall:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_column_id_ad_column:Option<AdColumn>,
	/// has one
	pub ad_reference_id_ad_reference:Option<AdReference>,
}

pub struct AdPackageImpDetail {
	/// primary
	/// not nullable 
	pub ad_package_imp_detail_id:f64,
	pub action:Option<String>,
	pub ad_backup_id:Option<f64>,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_original_id:f64,
	/// not nullable 
	pub ad_package_imp_id:f64,
	pub ad_table_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub name:Option<String>,
	pub success:Option<String>,
	pub tablename:Option<String>,
	pub type_:Option<String>,
	pub uninstall:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
}

pub struct AdPackageImpInst {
	/// primary
	/// not nullable 
	pub ad_package_imp_inst_id:f64,
	pub ad_client_id:Option<f64>,
	pub ad_org_id:Option<f64>,
	/// defaults to: now()
	pub created:Option<NaiveDateTime>,
	pub createdby:Option<f64>,
	pub createddate:Option<String>,
	pub creator:Option<String>,
	pub creatorcontact:Option<String>,
	pub description:Option<String>,
	pub email:Option<String>,
	/// defaults to: 'Y'::bpchar
	pub isactive:Option<String>,
	pub name:Option<String>,
	pub pk_status:Option<String>,
	pub pk_version:Option<String>,
	/// defaults to: 'N'::bpchar
	pub processed:Option<String>,
	/// defaults to: 'N'::bpchar
	pub processing:Option<String>,
	pub releaseno:Option<String>,
	pub uninstall:Option<String>,
	/// defaults to: now()
	pub updated:Option<NaiveDateTime>,
	pub updatedby:Option<f64>,
	pub updateddate:Option<String>,
	pub version:Option<String>,
}

pub struct AdPackageImpProc {
	/// primary
	/// not nullable 
	pub ad_package_imp_proc_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_override_dict:Option<String>,
	pub ad_package_dir:Option<String>,
	pub ad_package_source:Option<String>,
	/// not nullable 
	pub ad_package_source_type:String,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub processing:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
}

pub struct AdPinstance {
	/// primary
	/// not nullable 
	pub ad_pinstance_id:f64,
	pub ad_client_id:Option<f64>,
	pub ad_org_id:Option<f64>,
	/// not nullable 
	pub ad_process_id:f64,
	pub ad_user_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	pub createdby:Option<f64>,
	pub errormsg:Option<String>,
	/// defaults to: 'Y'::bpchar
	pub isactive:Option<String>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isprocessing:String,
	/// not nullable 
	pub record_id:f64,
	pub result:Option<f64>,
	/// defaults to: now()
	pub updated:Option<NaiveDateTime>,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_process_id_ad_process:Option<AdProcess>,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
	/// has many
	pub ad_pinstance_log:Option<Vec<AdPinstanceLog>>,
	/// has many
	pub ad_pinstance_para:Option<Vec<AdPinstancePara>>,
	/// has many
	pub t_aging:Option<Vec<TAging>>,
	/// has many
	pub t_inventoryvalue:Option<Vec<TInventoryvalue>>,
	/// has many
	pub t_invoicegl:Option<Vec<TInvoicegl>>,
	/// has many
	pub t_replenish:Option<Vec<TReplenish>>,
	/// has many
	pub t_report:Option<Vec<TReport>>,
	/// has many
	pub t_reportstatement:Option<Vec<TReportstatement>>,
	/// has many
	pub t_spool:Option<Vec<TSpool>>,
	/// has many
	pub t_transaction:Option<Vec<TTransaction>>,
	/// has many
	pub t_trialbalance:Option<Vec<TTrialbalance>>,
}

pub struct AdPinstanceLog {
	/// primary
	/// not nullable 
	pub ad_pinstance_id:f64,
	/// primary
	/// not nullable 
	pub log_id:f64,
	/// defaults to: now()
	pub p_date:Option<NaiveDateTime>,
	pub p_id:Option<f64>,
	pub p_msg:Option<String>,
	pub p_number:Option<f64>,
	/// has one
	pub ad_pinstance_id_ad_pinstance:Option<AdPinstance>,
}

pub struct AdPinstancePara {
	/// primary
	/// not nullable 
	pub ad_pinstance_id:f64,
	/// primary
	/// not nullable 
	pub seqno:f64,
	pub ad_client_id:Option<f64>,
	pub ad_org_id:Option<f64>,
	/// defaults to: now()
	pub created:Option<NaiveDateTime>,
	pub createdby:Option<f64>,
	pub info:Option<String>,
	pub info_to:Option<String>,
	/// defaults to: 'Y'::bpchar
	pub isactive:Option<String>,
	pub p_date:Option<NaiveDateTime>,
	pub p_date_to:Option<NaiveDateTime>,
	pub p_number:Option<f64>,
	pub p_number_to:Option<f64>,
	pub p_string:Option<String>,
	pub p_string_to:Option<String>,
	pub parametername:Option<String>,
	/// defaults to: now()
	pub updated:Option<NaiveDateTime>,
	pub updatedby:Option<f64>,
	/// has one
	pub ad_pinstance_id_ad_pinstance:Option<AdPinstance>,
}

pub struct AdPreference {
	/// primary
	/// not nullable 
	pub ad_preference_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_user_id:Option<f64>,
	pub ad_window_id:Option<f64>,
	/// not nullable 
	pub attribute:String,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
	/// has one
	pub ad_client_id_ad_client:Option<AdClient>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub ad_window_id_ad_window:Option<AdWindow>,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
}

pub struct AdPrintcolor {
	/// primary
	/// not nullable 
	pub ad_printcolor_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub code:String,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isdefault:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has many
	pub ad_orgtype:Option<Vec<AdOrgtype>>,
	/// has many
	pub ad_printformat:Option<Vec<AdPrintformat>>,
	/// has many
	pub ad_printformatitem:Option<Vec<AdPrintformatitem>>,
	/// has many
	pub ad_printtableformat:Option<Vec<AdPrinttableformat>>,
	/// has many
	pub c_bp_group:Option<Vec<CBpGroup>>,
	/// has many
	pub c_channel:Option<Vec<CChannel>>,
	/// has many
	pub c_poskey:Option<Vec<CPoskey>>,
	/// has many
	pub m_product_category:Option<Vec<MProductCategory>>,
	/// has many
	pub pa_colorschema:Option<Vec<PaColorschema>>,
}

pub struct AdPrintfont {
	/// primary
	/// not nullable 
	pub ad_printfont_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub code:String,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isdefault:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has many
	pub ad_printformat:Option<Vec<AdPrintformat>>,
	/// has many
	pub ad_printformatitem:Option<Vec<AdPrintformatitem>>,
	/// has many
	pub ad_printtableformat:Option<Vec<AdPrinttableformat>>,
}

pub struct AdPrintform {
	/// primary
	/// not nullable 
	pub ad_printform_id:f64,
	pub ad_client_id:Option<f64>,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub distrib_order_mailtext_id:Option<f64>,
	pub distrib_order_printformat_id:Option<f64>,
	pub invoice_mailtext_id:Option<f64>,
	pub invoice_printformat_id:Option<f64>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub manuf_order_mailtext_id:Option<f64>,
	pub manuf_order_printformat_id:Option<f64>,
	/// not nullable 
	pub name:String,
	pub order_mailtext_id:Option<f64>,
	pub order_printformat_id:Option<f64>,
	pub project_mailtext_id:Option<f64>,
	pub project_printformat_id:Option<f64>,
	pub remittance_mailtext_id:Option<f64>,
	pub remittance_printformat_id:Option<f64>,
	pub shipment_mailtext_id:Option<f64>,
	pub shipment_printformat_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_client_id_ad_client:Option<AdClient>,
	/// has one
	pub invoice_printformat_id_ad_printformat:Option<AdPrintformat>,
	/// has one
	pub order_printformat_id_ad_printformat:Option<AdPrintformat>,
	/// has one
	pub remittance_printformat_id_ad_printformat:Option<AdPrintformat>,
	/// has one
	pub shipment_printformat_id_ad_printformat:Option<AdPrintformat>,
	/// has one
	pub invoice_mailtext_id_r_mailtext:Option<RMailtext>,
	/// has one
	pub order_mailtext_id_r_mailtext:Option<RMailtext>,
	/// has one
	pub remittance_mailtext_id_r_mailtext:Option<RMailtext>,
	/// has one
	pub shipment_mailtext_id_r_mailtext:Option<RMailtext>,
	/// has one
	pub project_mailtext_id_r_mailtext:Option<RMailtext>,
	/// has one
	pub project_printformat_id_ad_printformat:Option<AdPrintformat>,
}

pub struct AdPrintformat {
	/// primary
	/// not nullable 
	pub ad_printformat_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_printcolor_id:f64,
	/// not nullable 
	pub ad_printfont_id:f64,
	/// not nullable 
	pub ad_printpaper_id:f64,
	pub ad_printtableformat_id:Option<f64>,
	pub ad_reportview_id:Option<f64>,
	/// not nullable 
	pub ad_table_id:f64,
	pub args:Option<String>,
	pub classname:Option<String>,
	pub createcopy:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// not nullable 
	pub footermargin:f64,
	/// not nullable 
	pub headermargin:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isdefault:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isform:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isstandardheaderfooter:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub istablebased:String,
	pub jasperprocess_id:Option<f64>,
	/// not nullable 
	pub name:String,
	pub printername:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_table_id_ad_table:Option<AdTable>,
	/// has one
	pub ad_printpaper_id_ad_printpaper:Option<AdPrintpaper>,
	/// has one
	pub ad_printcolor_id_ad_printcolor:Option<AdPrintcolor>,
	/// has one
	pub ad_printfont_id_ad_printfont:Option<AdPrintfont>,
	/// has one
	pub ad_reportview_id_ad_reportview:Option<AdReportview>,
	/// has one
	pub ad_printtableformat_id_ad_printtableformat:Option<AdPrinttableformat>,
	/// has one
	pub jasperprocess_id_ad_process:Option<AdProcess>,
	/// has many
	pub ad_package_exp_detail:Option<Vec<AdPackageExpDetail>>,
	/// has many
	pub ad_printform:Option<Vec<AdPrintform>>,
	/// has many
	pub ad_printformatitem:Option<Vec<AdPrintformatitem>>,
	/// has many
	pub ad_printgraph:Option<Vec<AdPrintgraph>>,
	/// has many
	pub ad_process:Option<Vec<AdProcess>>,
	/// has many
	pub c_bankaccountdoc:Option<Vec<CBankaccountdoc>>,
	/// has many
	pub c_bpartner:Option<Vec<CBpartner>>,
	/// has many
	pub c_doctype:Option<Vec<CDoctype>>,
	/// has many
	pub c_dunninglevel:Option<Vec<CDunninglevel>>,
	/// has many
	pub c_rfq_topic:Option<Vec<CRfqTopic>>,
	/// has many
	pub hr_payroll:Option<Vec<HrPayroll>>,
	/// has many
	pub hr_process:Option<Vec<HrProcess>>,
	/// has many
	pub pa_report:Option<Vec<PaReport>>,
}

pub struct AdPrintformatitem {
	/// primary
	/// not nullable 
	pub ad_printformatitem_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	pub ad_column_id:Option<f64>,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_printcolor_id:Option<f64>,
	pub ad_printfont_id:Option<f64>,
	/// not nullable 
	pub ad_printformat_id:f64,
	pub ad_printformatchild_id:Option<f64>,
	pub ad_printgraph_id:Option<f64>,
	pub arcdiameter:Option<f64>,
	pub barcodetype:Option<String>,
	pub belowcolumn:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub fieldalignmenttype:String,
	pub formatpattern:Option<String>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub imageisattached:String,
	pub imageurl:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isaveraged:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub iscentrallymaintained:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub iscounted:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isdeviationcalc:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isfilledrectangle:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isfixedwidth:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isgroupby:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isheightoneline:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isimagefield:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub ismaxcalc:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub ismincalc:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isnextline:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isnextpage:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isorderby:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub ispagebreak:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isprinted:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isrelativeposition:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isrunningtotal:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub issetnlposition:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub issummarized:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub issuppressnull:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub issuppressrepeats:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isvariancecalc:String,
	/// not nullable 
	pub linealignmenttype:String,
	pub linewidth:Option<f64>,
	/// not nullable 
	pub maxheight:f64,
	/// not nullable 
	pub maxwidth:f64,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub printareatype:String,
	/// not nullable 
	pub printformattype:String,
	pub printname:Option<String>,
	pub printnamesuffix:Option<String>,
	pub runningtotallines:Option<f64>,
	/// not nullable 
	pub seqno:f64,
	pub shapetype:Option<String>,
	/// not nullable 
	pub sortno:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub xposition:f64,
	/// not nullable 
	pub xspace:f64,
	/// not nullable 
	pub yposition:f64,
	/// not nullable 
	pub yspace:f64,
	/// has one
	pub ad_printformat_id_ad_printformat:Option<AdPrintformat>,
	/// has one
	pub ad_column_id_ad_column:Option<AdColumn>,
	/// has one
	pub ad_printformatchild_id_ad_printformat:Option<AdPrintformat>,
	/// has one
	pub ad_printcolor_id_ad_printcolor:Option<AdPrintcolor>,
	/// has one
	pub ad_printfont_id_ad_printfont:Option<AdPrintfont>,
	/// has one
	pub ad_printgraph_id_ad_printgraph:Option<AdPrintgraph>,
	/// has many
	pub ad_printformatitem_trl:Option<Vec<AdPrintformatitemTrl>>,
	/// has many
	pub ad_printgraph:Option<Vec<AdPrintgraph>>,
}

pub struct AdPrintformatitemTrl {
	/// primary
	/// not nullable 
	pub ad_printformatitem_id:f64,
	/// primary
	/// not nullable 
	pub ad_language:String,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub istranslated:String,
	pub printname:Option<String>,
	pub printnamesuffix:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_printformatitem_id_ad_printformatitem:Option<AdPrintformatitem>,
	/// has one
	pub ad_language_ad_language:Option<AdLanguage>,
}

pub struct AdPrintgraph {
	/// primary
	/// not nullable 
	pub ad_printgraph_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_printformat_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub data1_printformatitem_id:Option<f64>,
	pub data2_printformatitem_id:Option<f64>,
	pub data3_printformatitem_id:Option<f64>,
	pub data4_printformatitem_id:Option<f64>,
	/// not nullable 
	pub data_printformatitem_id:f64,
	pub description:Option<String>,
	/// not nullable 
	pub description_printformatitem_id:f64,
	/// not nullable 
	pub graphtype:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub description_printformatitem_id_ad_printformatitem:Option<AdPrintformatitem>,
	/// has one
	pub data_printformatitem_id_ad_printformatitem:Option<AdPrintformatitem>,
	/// has one
	pub data1_printformatitem_id_ad_printformatitem:Option<AdPrintformatitem>,
	/// has one
	pub data2_printformatitem_id_ad_printformatitem:Option<AdPrintformatitem>,
	/// has one
	pub data3_printformatitem_id_ad_printformatitem:Option<AdPrintformatitem>,
	/// has one
	pub data4_printformatitem_id_ad_printformatitem:Option<AdPrintformatitem>,
	/// has one
	pub ad_printformat_id_ad_printformat:Option<AdPrintformat>,
	/// has many
	pub ad_printformatitem:Option<Vec<AdPrintformatitem>>,
}

pub struct AdPrintlabel {
	/// primary
	/// not nullable 
	pub ad_printlabel_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_labelprinter_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_table_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub islandscape:String,
	/// not nullable 
	pub labelheight:f64,
	/// not nullable 
	pub labelwidth:f64,
	/// not nullable 
	pub name:String,
	pub printername:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_table_id_ad_table:Option<AdTable>,
	/// has one
	pub ad_labelprinter_id_ad_labelprinter:Option<AdLabelprinter>,
	/// has many
	pub ad_printlabelline:Option<Vec<AdPrintlabelline>>,
}

pub struct AdPrintlabelline {
	/// primary
	/// not nullable 
	pub ad_printlabelline_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	pub ad_column_id:Option<f64>,
	/// not nullable 
	pub ad_labelprinterfunction_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_printlabel_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub labelformattype:String,
	/// not nullable 
	pub name:String,
	pub printname:Option<String>,
	/// not nullable 
	pub seqno:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub xposition:f64,
	/// not nullable 
	pub yposition:f64,
	/// has one
	pub ad_printlabel_id_ad_printlabel:Option<AdPrintlabel>,
	/// has one
	pub ad_column_id_ad_column:Option<AdColumn>,
	/// has one
	pub ad_labelprinterfunction_id_ad_labelprinterfunction:Option<AdLabelprinterfunction>,
	/// has many
	pub ad_printlabelline_trl:Option<Vec<AdPrintlabellineTrl>>,
}

pub struct AdPrintlabellineTrl {
	/// primary
	/// not nullable 
	pub ad_printlabelline_id:f64,
	/// primary
	/// not nullable 
	pub ad_language:String,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub istranslated:String,
	pub printname:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_printlabelline_id_ad_printlabelline:Option<AdPrintlabelline>,
	/// has one
	pub ad_language_ad_language:Option<AdLanguage>,
}

pub struct AdPrintpaper {
	/// primary
	/// not nullable 
	pub ad_printpaper_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub code:String,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub dimensionunits:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isdefault:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub islandscape:String,
	/// defaults to: 36
	/// not nullable 
	pub marginbottom:f64,
	/// defaults to: 36
	/// not nullable 
	pub marginleft:f64,
	/// defaults to: 36
	/// not nullable 
	pub marginright:f64,
	/// defaults to: 36
	/// not nullable 
	pub margintop:f64,
	/// not nullable 
	pub name:String,
	pub processing:Option<String>,
	pub sizex:Option<f64>,
	pub sizey:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has many
	pub ad_printformat:Option<Vec<AdPrintformat>>,
}

pub struct AdPrinttableformat {
	/// primary
	/// not nullable 
	pub ad_printtableformat_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	pub ad_image_id:Option<f64>,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub footercenter:Option<String>,
	pub footerleft:Option<String>,
	pub footerright:Option<String>,
	pub funct_printfont_id:Option<f64>,
	pub functbg_printcolor_id:Option<f64>,
	pub functfg_printcolor_id:Option<f64>,
	pub hdr_printfont_id:Option<f64>,
	pub hdrline_printcolor_id:Option<f64>,
	pub hdrstroke:Option<f64>,
	pub hdrstroketype:Option<String>,
	pub hdrtextbg_printcolor_id:Option<f64>,
	pub hdrtextfg_printcolor_id:Option<f64>,
	pub headercenter:Option<String>,
	pub headerleft:Option<String>,
	pub headerright:Option<String>,
	/// defaults to: 'N'::bpchar
	pub imageisattached:Option<String>,
	pub imageurl:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isdefault:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub ismultilineheader:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub ispaintboundarylines:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub ispaintheaderlines:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub ispainthlines:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub ispaintvlines:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isprintfunctionsymbols:String,
	pub line_printcolor_id:Option<f64>,
	pub linestroke:Option<f64>,
	pub linestroketype:Option<String>,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub hdr_printfont_id_ad_printfont:Option<AdPrintfont>,
	/// has one
	pub hdrtextfg_printcolor_id_ad_printcolor:Option<AdPrintcolor>,
	/// has one
	pub hdrtextbg_printcolor_id_ad_printcolor:Option<AdPrintcolor>,
	/// has one
	pub hdrline_printcolor_id_ad_printcolor:Option<AdPrintcolor>,
	/// has one
	pub funct_printfont_id_ad_printfont:Option<AdPrintfont>,
	/// has one
	pub functbg_printcolor_id_ad_printcolor:Option<AdPrintcolor>,
	/// has one
	pub functfg_printcolor_id_ad_printcolor:Option<AdPrintcolor>,
	/// has one
	pub line_printcolor_id_ad_printcolor:Option<AdPrintcolor>,
	/// has many
	pub ad_printformat:Option<Vec<AdPrintformat>>,
}

pub struct AdPrivateAccess {
	/// primary
	/// not nullable 
	pub ad_user_id:f64,
	/// primary
	/// not nullable 
	pub ad_table_id:f64,
	/// primary
	/// not nullable 
	pub record_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
	/// has one
	pub ad_table_id_ad_table:Option<AdTable>,
}

pub struct AdProcess {
	/// primary
	/// not nullable 
	pub ad_process_id:f64,
	/// not nullable 
	pub accesslevel:String,
	/// not nullable 
	pub ad_client_id:f64,
	pub ad_form_id:Option<f64>,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_printformat_id:Option<f64>,
	pub ad_reportview_id:Option<f64>,
	pub ad_workflow_id:Option<f64>,
	pub classname:Option<String>,
	/// defaults to: NULL::bpchar
	pub copyfromprocess:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'D'::character varying
	/// not nullable 
	pub entitytype:String,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isbetafunctionality:String,
	/// defaults to: 'N'::bpchar
	pub isdirectprint:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isreport:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isserverprocess:String,
	pub jasperreport:Option<String>,
	/// not nullable 
	pub name:String,
	pub procedurename:Option<String>,
	/// defaults to: 'Y'::bpchar
	pub showhelp:Option<String>,
	pub statistic_count:Option<f64>,
	pub statistic_seconds:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
	pub workflowvalue:Option<String>,
	/// has one
	pub entitytype_ad_entitytype:Option<AdEntitytype>,
	/// has one
	pub ad_reportview_id_ad_reportview:Option<AdReportview>,
	/// has one
	pub ad_printformat_id_ad_printformat:Option<AdPrintformat>,
	/// has one
	pub ad_workflow_id_ad_workflow:Option<AdWorkflow>,
	/// has many
	pub ad_archive:Option<Vec<AdArchive>>,
	/// has many
	pub ad_column:Option<Vec<AdColumn>>,
	/// has many
	pub ad_issue:Option<Vec<AdIssue>>,
	/// has many
	pub ad_menu:Option<Vec<AdMenu>>,
	/// has many
	pub ad_package_exp_common:Option<Vec<AdPackageExpCommon>>,
	/// has many
	pub ad_package_exp_detail:Option<Vec<AdPackageExpDetail>>,
	/// has many
	pub ad_pinstance:Option<Vec<AdPinstance>>,
	/// has many
	pub ad_printformat:Option<Vec<AdPrintformat>>,
	/// has many
	pub ad_process_access:Option<Vec<AdProcessAccess>>,
	/// has many
	pub ad_process_para:Option<Vec<AdProcessPara>>,
	/// has many
	pub ad_process_trl:Option<Vec<AdProcessTrl>>,
	/// has many
	pub ad_scheduler:Option<Vec<AdScheduler>>,
	/// has many
	pub ad_tab:Option<Vec<AdTab>>,
	/// has many
	pub ad_wf_node:Option<Vec<AdWfNode>>,
	/// has many
	pub ad_workbenchwindow:Option<Vec<AdWorkbenchwindow>>,
	/// has many
	pub asp_clientexception:Option<Vec<AspClientexception>>,
	/// has many
	pub asp_process:Option<Vec<AspProcess>>,
	/// has many
	pub pa_report:Option<Vec<PaReport>>,
	/// has many
	pub pp_order_node:Option<Vec<PpOrderNode>>,
}

pub struct AdProcessAccess {
	/// primary
	/// not nullable 
	pub ad_process_id:f64,
	/// primary
	/// not nullable 
	pub ad_role_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isreadwrite:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_process_id_ad_process:Option<AdProcess>,
	/// has one
	pub ad_role_id_ad_role:Option<AdRole>,
	/// has one
	pub ad_client_id_ad_client:Option<AdClient>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
}

pub struct AdProcessPara {
	/// primary
	/// not nullable 
	pub ad_process_para_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	pub ad_element_id:Option<f64>,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_process_id:f64,
	/// not nullable 
	pub ad_reference_id:f64,
	pub ad_reference_value_id:Option<f64>,
	pub ad_val_rule_id:Option<f64>,
	/// not nullable 
	pub columnname:String,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub defaultvalue:Option<String>,
	pub defaultvalue2:Option<String>,
	pub description:Option<String>,
	pub displaylogic:Option<String>,
	/// defaults to: 'D'::character varying
	/// not nullable 
	pub entitytype:String,
	/// not nullable 
	pub fieldlength:f64,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub iscentrallymaintained:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub ismandatory:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isrange:String,
	/// not nullable 
	pub name:String,
	pub readonlylogic:Option<String>,
	/// not nullable 
	pub seqno:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub valuemax:Option<String>,
	pub valuemin:Option<String>,
	pub vformat:Option<String>,
	/// has one
	pub ad_process_id_ad_process:Option<AdProcess>,
	/// has one
	pub ad_reference_id_ad_reference:Option<AdReference>,
	/// has one
	pub ad_reference_value_id_ad_reference:Option<AdReference>,
	/// has one
	pub ad_val_rule_id_ad_val_rule:Option<AdValRule>,
	/// has one
	pub ad_element_id_ad_element:Option<AdElement>,
	/// has one
	pub entitytype_ad_entitytype:Option<AdEntitytype>,
	/// has many
	pub ad_process_para_trl:Option<Vec<AdProcessParaTrl>>,
	/// has many
	pub ad_scheduler_para:Option<Vec<AdSchedulerPara>>,
	/// has many
	pub ad_wf_node_para:Option<Vec<AdWfNodePara>>,
	/// has many
	pub asp_clientexception:Option<Vec<AspClientexception>>,
	/// has many
	pub asp_process_para:Option<Vec<AspProcessPara>>,
}

pub struct AdProcessParaTrl {
	/// primary
	/// not nullable 
	pub ad_process_para_id:f64,
	/// primary
	/// not nullable 
	pub ad_language:String,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub istranslated:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_process_para_id_ad_process_para:Option<AdProcessPara>,
	/// has one
	pub ad_language_ad_language:Option<AdLanguage>,
}

pub struct AdProcessTrl {
	/// primary
	/// not nullable 
	pub ad_process_id:f64,
	/// primary
	/// not nullable 
	pub ad_language:String,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub istranslated:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_process_id_ad_process:Option<AdProcess>,
	/// has one
	pub ad_language_ad_language:Option<AdLanguage>,
}

pub struct AdRecordAccess {
	/// primary
	/// not nullable 
	pub ad_role_id:f64,
	/// primary
	/// not nullable 
	pub ad_table_id:f64,
	/// primary
	/// not nullable 
	pub record_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isdependententities:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isexclude:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isreadonly:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_role_id_ad_role:Option<AdRole>,
	/// has one
	pub ad_table_id_ad_table:Option<AdTable>,
}

pub struct AdRefList {
	/// primary
	/// not nullable 
	pub ad_ref_list_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_reference_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'D'::character varying
	/// not nullable 
	pub entitytype:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub validfrom:Option<NaiveDateTime>,
	pub validto:Option<NaiveDateTime>,
	/// not nullable 
	pub value:String,
	/// has one
	pub ad_client_id_ad_client:Option<AdClient>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub ad_reference_id_ad_reference:Option<AdReference>,
	/// has one
	pub entitytype_ad_entitytype:Option<AdEntitytype>,
	/// has many
	pub ad_document_action_access:Option<Vec<AdDocumentActionAccess>>,
	/// has many
	pub ad_ref_list_trl:Option<Vec<AdRefListTrl>>,
}

pub struct AdRefListTrl {
	/// primary
	/// not nullable 
	pub ad_ref_list_id:f64,
	/// primary
	/// not nullable 
	pub ad_language:String,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub istranslated:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_ref_list_id_ad_ref_list:Option<AdRefList>,
	/// has one
	pub ad_language_ad_language:Option<AdLanguage>,
}

pub struct AdRefTable {
	/// primary
	/// not nullable 
	pub ad_reference_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_display:f64,
	/// not nullable 
	pub ad_key:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_ref_table_id:Option<f64>,
	/// not nullable 
	pub ad_table_id:f64,
	pub ad_window_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'D'::character varying
	/// not nullable 
	pub entitytype:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isvaluedisplayed:String,
	pub orderbyclause:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub whereclause:Option<String>,
	/// has one
	pub ad_reference_id_ad_reference:Option<AdReference>,
	/// has one
	pub ad_client_id_ad_client:Option<AdClient>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub ad_table_id_ad_table:Option<AdTable>,
	/// has one
	pub ad_key_ad_column:Option<AdColumn>,
	/// has one
	pub ad_display_ad_column:Option<AdColumn>,
	/// has one
	pub entitytype_ad_entitytype:Option<AdEntitytype>,
	/// has one
	pub ad_window_id_ad_window:Option<AdWindow>,
}

pub struct AdReference {
	/// primary
	/// not nullable 
	pub ad_reference_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'D'::character varying
	/// not nullable 
	pub entitytype:String,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	pub isorderbyvalue:Option<String>,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub validationtype:String,
	pub vformat:Option<String>,
	/// has one
	pub ad_client_id_ad_client:Option<AdClient>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub entitytype_ad_entitytype:Option<AdEntitytype>,
	/// has many
	pub a_registrationattribute:Option<Vec<ARegistrationattribute>>,
	/// has many
	pub ad_attribute:Option<Vec<AdAttribute>>,
	/// has many
	pub ad_column:Option<Vec<AdColumn>>,
	/// has many
	pub ad_field:Option<Vec<AdField>>,
	/// has many
	pub ad_infocolumn:Option<Vec<AdInfocolumn>>,
	/// has many
	pub ad_package_exp_detail:Option<Vec<AdPackageExpDetail>>,
	/// has many
	pub ad_package_imp_backup:Option<Vec<AdPackageImpBackup>>,
	/// has many
	pub ad_process_para:Option<Vec<AdProcessPara>>,
	/// has many
	pub ad_ref_list:Option<Vec<AdRefList>>,
	/// has many
	pub ad_ref_table:Option<Vec<AdRefTable>>,
	/// has many
	pub ad_reference_trl:Option<Vec<AdReferenceTrl>>,
	/// has many
	pub hr_concept:Option<Vec<HrConcept>>,
}

pub struct AdReferenceTrl {
	/// primary
	/// not nullable 
	pub ad_reference_id:f64,
	/// primary
	/// not nullable 
	pub ad_language:String,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	pub istranslated:Option<String>,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_reference_id_ad_reference:Option<AdReference>,
	/// has one
	pub ad_language_ad_language:Option<AdLanguage>,
}

pub struct AdRegistration {
	/// primary
	/// not nullable 
	pub ad_registration_id:f64,
	/// primary
	/// not nullable 
	pub ad_client_id:f64,
	/// primary
	/// not nullable 
	pub ad_system_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub c_currency_id:Option<f64>,
	pub c_location_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub industryinfo:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isallowpublish:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isallowstatistics:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isinproduction:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isregistered:String,
	/// defaults to: 0
	pub numberemployees:Option<f64>,
	pub platforminfo:Option<String>,
	pub processing:Option<String>,
	pub record_id:Option<f64>,
	pub remote_addr:Option<String>,
	pub remote_host:Option<String>,
	/// defaults to: 0
	pub salesvolume:Option<f64>,
	pub startproductiondate:Option<NaiveDateTime>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_client_id_ad_system:Option<AdSystem>,
	/// has one
	pub ad_system_id_ad_system:Option<AdSystem>,
	/// has one
	pub c_currency_id_c_currency:Option<CCurrency>,
}

pub struct AdReplication {
	/// primary
	/// not nullable 
	pub ad_replication_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_replicationstrategy_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub datelastrun:Option<NaiveDateTime>,
	pub description:Option<String>,
	pub help:Option<String>,
	/// not nullable 
	pub hostaddress:String,
	/// not nullable 
	pub hostport:f64,
	pub idrangeend:Option<f64>,
	pub idrangestart:Option<f64>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isrmioverhttp:String,
	/// not nullable 
	pub name:String,
	pub prefix:Option<String>,
	pub processing:Option<String>,
	pub remote_client_id:Option<f64>,
	pub remote_org_id:Option<f64>,
	pub suffix:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_replicationstrategy_id_ad_replicationstrategy:Option<AdReplicationstrategy>,
	/// has one
	pub remote_client_id_ad_client:Option<AdClient>,
	/// has one
	pub remote_org_id_ad_org:Option<AdOrg>,
	/// has many
	pub ad_replication_run:Option<Vec<AdReplicationRun>>,
}

pub struct AdReplicationLog {
	/// primary
	/// not nullable 
	pub ad_replication_log_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_replication_run_id:f64,
	pub ad_replicationtable_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isreplicated:String,
	pub p_msg:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_replication_run_id_ad_replication_run:Option<AdReplicationRun>,
	/// has one
	pub ad_replicationtable_id_ad_replicationtable:Option<AdReplicationtable>,
}

pub struct AdReplicationRun {
	/// primary
	/// not nullable 
	pub ad_replication_run_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_replication_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isreplicated:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_replication_id_ad_replication:Option<AdReplication>,
	/// has many
	pub ad_replication_log:Option<Vec<AdReplicationLog>>,
}

pub struct AdReplicationdocument {
	/// primary
	/// not nullable 
	pub ad_replicationdocument_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_replicationstrategy_id:f64,
	/// not nullable 
	pub ad_table_id:f64,
	/// not nullable 
	pub c_doctype_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub replicationtype:String,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_doctype_id_c_doctype:Option<CDoctype>,
	/// has one
	pub ad_table_id_ad_table:Option<AdTable>,
}

pub struct AdReplicationstrategy {
	/// primary
	/// not nullable 
	pub ad_replicationstrategy_id:f64,
	/// unique
	/// not nullable 
	pub ad_client_id:f64,
	/// unique
	pub value:Option<String>,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'D'::character varying
	/// not nullable 
	pub entitytype:String,
	pub exp_processor_id:Option<f64>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub entitytype_ad_entitytype:Option<AdEntitytype>,
	/// has one
	pub exp_processor_id_exp_processor:Option<ExpProcessor>,
	/// has many
	pub ad_client:Option<Vec<AdClient>>,
	/// has many
	pub ad_org:Option<Vec<AdOrg>>,
	/// has many
	pub ad_replication:Option<Vec<AdReplication>>,
	/// has many
	pub ad_replicationtable:Option<Vec<AdReplicationtable>>,
}

pub struct AdReplicationtable {
	/// primary
	/// not nullable 
	pub ad_replicationtable_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_replicationstrategy_id:f64,
	/// not nullable 
	pub ad_table_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'D'::character varying
	/// not nullable 
	pub entitytype:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'L'::bpchar
	/// not nullable 
	pub replicationtype:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_replicationstrategy_id_ad_replicationstrategy:Option<AdReplicationstrategy>,
	/// has one
	pub ad_table_id_ad_table:Option<AdTable>,
	/// has one
	pub entitytype_ad_entitytype:Option<AdEntitytype>,
	/// has many
	pub ad_replication_log:Option<Vec<AdReplicationLog>>,
}

pub struct AdReportview {
	/// primary
	/// not nullable 
	pub ad_reportview_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_table_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'D'::character varying
	/// not nullable 
	pub entitytype:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	pub orderbyclause:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub whereclause:Option<String>,
	/// has one
	pub ad_table_id_ad_table:Option<AdTable>,
	/// has one
	pub entitytype_ad_entitytype:Option<AdEntitytype>,
	/// has many
	pub ad_package_exp_common:Option<Vec<AdPackageExpCommon>>,
	/// has many
	pub ad_package_exp_detail:Option<Vec<AdPackageExpDetail>>,
	/// has many
	pub ad_printformat:Option<Vec<AdPrintformat>>,
	/// has many
	pub ad_process:Option<Vec<AdProcess>>,
	/// has many
	pub ad_reportview_col:Option<Vec<AdReportviewCol>>,
}

pub struct AdReportviewCol {
	/// primary
	/// not nullable 
	pub ad_reportview_col_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	pub ad_column_id:Option<f64>,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_reportview_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub functioncolumn:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isgroupfunction:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_reportview_id_ad_reportview:Option<AdReportview>,
	/// has one
	pub ad_column_id_ad_column:Option<AdColumn>,
}

pub struct AdRole {
	/// primary
	/// not nullable 
	pub ad_role_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_tree_menu_id:Option<f64>,
	pub ad_tree_org_id:Option<f64>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub allow_info_account:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub allow_info_asset:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub allow_info_bpartner:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub allow_info_cashjournal:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub allow_info_crp:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub allow_info_inout:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub allow_info_invoice:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub allow_info_mrp:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub allow_info_order:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub allow_info_payment:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub allow_info_product:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub allow_info_resource:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub allow_info_schedule:String,
	/// defaults to: 0
	pub amtapproval:Option<f64>,
	pub c_currency_id:Option<f64>,
	/// defaults to: 0
	/// not nullable 
	pub confirmqueryrecords:f64,
	pub connectionprofile:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isaccessallorgs:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub iscanapproveowndoc:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub iscanexport:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub iscanreport:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub ischangelog:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isdiscountallowedontotal:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isdiscountuptolimitprice:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub ismanual:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub ispersonalaccess:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub ispersonallock:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isshowacct:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isuseuserorgaccess:String,
	/// defaults to: 0
	/// not nullable 
	pub maxqueryrecords:f64,
	/// not nullable 
	pub name:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub overwritepricelimit:String,
	/// defaults to: 'C'::bpchar
	/// not nullable 
	pub preferencetype:String,
	pub supervisor_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub userdiscount:Option<f64>,
	/// defaults to: '  O'::bpchar
	/// not nullable 
	pub userlevel:String,
	/// has one
	pub ad_client_id_ad_client:Option<AdClient>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub c_currency_id_c_currency:Option<CCurrency>,
	/// has one
	pub ad_tree_menu_id_ad_tree:Option<AdTree>,
	/// has one
	pub supervisor_id_ad_user:Option<AdUser>,
	/// has one
	pub ad_tree_org_id_ad_tree:Option<AdTree>,
	/// has many
	pub ad_alertrecipient:Option<Vec<AdAlertrecipient>>,
	/// has many
	pub ad_column_access:Option<Vec<AdColumnAccess>>,
	/// has many
	pub ad_document_action_access:Option<Vec<AdDocumentActionAccess>>,
	/// has many
	pub ad_form_access:Option<Vec<AdFormAccess>>,
	/// has many
	pub ad_package_exp_common:Option<Vec<AdPackageExpCommon>>,
	/// has many
	pub ad_package_exp_detail:Option<Vec<AdPackageExpDetail>>,
	/// has many
	pub ad_process_access:Option<Vec<AdProcessAccess>>,
	/// has many
	pub ad_record_access:Option<Vec<AdRecordAccess>>,
	/// has many
	pub ad_role_orgaccess:Option<Vec<AdRoleOrgaccess>>,
	/// has many
	pub ad_schedulerrecipient:Option<Vec<AdSchedulerrecipient>>,
	/// has many
	pub ad_session:Option<Vec<AdSession>>,
	/// has many
	pub ad_table_access:Option<Vec<AdTableAccess>>,
	/// has many
	pub ad_task_access:Option<Vec<AdTaskAccess>>,
	/// has many
	pub ad_user_roles:Option<Vec<AdUserRoles>>,
	/// has many
	pub ad_userdef_win:Option<Vec<AdUserdefWin>>,
	/// has many
	pub ad_wf_responsible:Option<Vec<AdWfResponsible>>,
	/// has many
	pub ad_window_access:Option<Vec<AdWindowAccess>>,
	/// has many
	pub ad_workflow_access:Option<Vec<AdWorkflowAccess>>,
	/// has many
	pub cm_accesslistrole:Option<Vec<CmAccesslistrole>>,
	/// has many
	pub pa_goal:Option<Vec<PaGoal>>,
	/// has many
	pub r_request:Option<Vec<RRequest>>,
	/// has many
	pub r_requestaction:Option<Vec<RRequestaction>>,
	/// has many
	pub u_rolemenu:Option<Vec<URolemenu>>,
}

pub struct AdRoleOrgaccess {
	/// primary
	/// not nullable 
	pub ad_role_id:f64,
	/// primary
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isreadonly:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_role_id_ad_role:Option<AdRole>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
}

pub struct AdRule {
	/// primary
	/// not nullable 
	pub ad_rule_id:f64,
	pub accesslevel:Option<String>,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'U'::character varying
	pub entitytype:Option<String>,
	pub eventtype:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	pub ruletype:Option<String>,
	pub script:Option<String>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
	/// has one
	pub entitytype_ad_entitytype:Option<AdEntitytype>,
	/// has many
	pub ad_table_scriptvalidator:Option<Vec<AdTableScriptvalidator>>,
	/// has many
	pub c_tax:Option<Vec<CTax>>,
	/// has many
	pub hr_attribute:Option<Vec<HrAttribute>>,
	/// has many
	pub hr_movement:Option<Vec<HrMovement>>,
	/// has many
	pub hr_payrollconcept:Option<Vec<HrPayrollconcept>>,
}

pub struct AdScheduler {
	/// primary
	/// not nullable 
	pub ad_scheduler_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_process_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub datelastrun:Option<NaiveDateTime>,
	pub datenextrun:Option<NaiveDateTime>,
	pub description:Option<String>,
	/// not nullable 
	pub frequency:f64,
	/// not nullable 
	pub frequencytype:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub keeplogdays:f64,
	pub monthday:Option<f64>,
	/// not nullable 
	pub name:String,
	pub processing:Option<String>,
	/// defaults to: 'F'::bpchar
	/// not nullable 
	pub scheduletype:String,
	/// not nullable 
	pub supervisor_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub weekday:Option<String>,
	/// has one
	pub ad_process_id_ad_process:Option<AdProcess>,
	/// has one
	pub supervisor_id_ad_user:Option<AdUser>,
	/// has many
	pub ad_scheduler_para:Option<Vec<AdSchedulerPara>>,
	/// has many
	pub ad_schedulerlog:Option<Vec<AdSchedulerlog>>,
	/// has many
	pub ad_schedulerrecipient:Option<Vec<AdSchedulerrecipient>>,
}

pub struct AdSchedulerPara {
	/// primary
	/// not nullable 
	pub ad_scheduler_id:f64,
	/// primary
	/// not nullable 
	pub ad_process_para_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub parameterdefault:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_scheduler_id_ad_scheduler:Option<AdScheduler>,
	/// has one
	pub ad_process_para_id_ad_process_para:Option<AdProcessPara>,
}

pub struct AdSchedulerlog {
	/// primary
	/// not nullable 
	pub ad_scheduler_id:f64,
	/// primary
	/// not nullable 
	pub ad_schedulerlog_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub binarydata:Option<Vec<u8>>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub iserror:String,
	pub reference:Option<String>,
	pub summary:Option<String>,
	pub textmsg:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_scheduler_id_ad_scheduler:Option<AdScheduler>,
}

pub struct AdSchedulerrecipient {
	/// primary
	/// not nullable 
	pub ad_schedulerrecipient_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_role_id:Option<f64>,
	/// not nullable 
	pub ad_scheduler_id:f64,
	pub ad_user_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_scheduler_id_ad_scheduler:Option<AdScheduler>,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
	/// has one
	pub ad_role_id_ad_role:Option<AdRole>,
}

pub struct AdSearchdefinition {
	/// primary
	/// not nullable 
	pub ad_searchdefinition_id:f64,
	/// defaults to: NULL::numeric
	/// not nullable 
	pub ad_client_id:f64,
	pub ad_column_id:Option<f64>,
	/// defaults to: NULL::numeric
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_table_id:f64,
	/// not nullable 
	pub ad_window_id:f64,
	pub created:Option<NaiveDateTime>,
	pub createdby:Option<f64>,
	/// not nullable 
	pub datatype:String,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isdefault:String,
	pub name:Option<String>,
	pub po_window_id:Option<f64>,
	pub query:Option<String>,
	/// not nullable 
	pub searchtype:String,
	pub transactioncode:Option<String>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
}

pub struct AdSequence {
	/// primary
	/// not nullable 
	pub ad_sequence_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub currentnext:f64,
	/// not nullable 
	pub currentnextsys:f64,
	pub datecolumn:Option<String>,
	pub decimalpattern:Option<String>,
	pub description:Option<String>,
	/// not nullable 
	pub incrementno:f64,
	/// defaults to: 'Y'::bpchar
	pub isactive:Option<String>,
	/// defaults to: 'N'::bpchar
	pub isaudited:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isautosequence:String,
	/// defaults to: 'N'::bpchar
	pub istableid:Option<String>,
	/// not nullable 
	pub name:String,
	pub prefix:Option<String>,
	/// defaults to: 'N'::bpchar
	pub startnewyear:Option<String>,
	/// not nullable 
	pub startno:f64,
	pub suffix:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub vformat:Option<String>,
	/// has one
	pub ad_client_id_ad_client:Option<AdClient>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has many
	pub ad_sequence_audit:Option<Vec<AdSequenceAudit>>,
	/// has many
	pub ad_sequence_no:Option<Vec<AdSequenceNo>>,
	/// has many
	pub c_bp_edi:Option<Vec<CBpEdi>>,
	/// has many
	pub c_doctype:Option<Vec<CDoctype>>,
	/// has many
	pub c_paymentprocessor:Option<Vec<CPaymentprocessor>>,
}

pub struct AdSequenceAudit {
	/// primary
	/// not nullable 
	pub ad_sequence_id:f64,
	/// primary
	/// not nullable 
	pub documentno:String,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_table_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub record_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_sequence_id_ad_sequence:Option<AdSequence>,
	/// has one
	pub ad_client_id_ad_client:Option<AdClient>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub ad_table_id_ad_table:Option<AdTable>,
}

pub struct AdSequenceNo {
	/// primary
	/// not nullable 
	pub ad_sequence_id:f64,
	/// primary
	/// defaults to: '0000'::character varying
	/// not nullable 
	pub calendaryear:String,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub currentnext:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_sequence_id_ad_sequence:Option<AdSequence>,
	/// has one
	pub ad_client_id_ad_client:Option<AdClient>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
}

pub struct AdSession {
	/// primary
	/// not nullable 
	pub ad_session_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_role_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub logindate:Option<NaiveDateTime>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	pub remote_addr:Option<String>,
	pub remote_host:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub websession:Option<String>,
	/// has one
	pub ad_role_id_ad_role:Option<AdRole>,
	/// has many
	pub ad_changelog:Option<Vec<AdChangelog>>,
	/// has many
	pub k_comment:Option<Vec<KComment>>,
	/// has many
	pub k_entry:Option<Vec<KEntry>>,
}

pub struct AdSysconfig {
	/// primary
	/// not nullable 
	pub ad_sysconfig_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: 'S'::bpchar
	pub configurationlevel:Option<String>,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'U'::character varying
	/// not nullable 
	pub entitytype:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
	/// has one
	pub entitytype_ad_entitytype:Option<AdEntitytype>,
}

pub struct AdSystem {
	/// primary
	/// not nullable 
	pub ad_system_id:f64,
	/// primary
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub customprefix:Option<String>,
	pub dbaddress:Option<String>,
	pub dbinstance:Option<String>,
	pub description:Option<String>,
	pub encryptionkey:Option<String>,
	pub idrangeend:Option<f64>,
	pub idrangestart:Option<f64>,
	pub info:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isallowstatistics:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isautoerrorreport:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isfailonbuilddiffer:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isfailonmissingmodelvalidator:String,
	/// defaults to: 'N'::bpchar
	pub isjustmigrated:Option<String>,
	pub lastbuildinfo:Option<String>,
	pub ldapdomain:Option<String>,
	pub ldaphost:Option<String>,
	/// not nullable 
	pub name:String,
	pub noprocessors:Option<f64>,
	pub oldname:Option<String>,
	pub password:Option<String>,
	pub processing:Option<String>,
	pub profileinfo:Option<String>,
	pub record_id:Option<f64>,
	/// not nullable 
	pub releaseno:String,
	/// defaults to: 'L'::bpchar
	/// not nullable 
	pub replicationtype:String,
	pub statisticsinfo:Option<String>,
	pub summary:Option<String>,
	pub supportemail:Option<String>,
	pub supportexpdate:Option<NaiveDateTime>,
	pub supportunits:Option<f64>,
	/// defaults to: 'E'::bpchar
	/// not nullable 
	pub systemstatus:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub username:String,
	pub version:Option<String>,
	/// has many
	pub ad_registration:Option<Vec<AdRegistration>>,
}

pub struct AdTab {
	/// primary
	/// not nullable 
	pub ad_tab_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	pub ad_column_id:Option<f64>,
	pub ad_columnsortorder_id:Option<f64>,
	pub ad_columnsortyesno_id:Option<f64>,
	pub ad_image_id:Option<f64>,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_process_id:Option<f64>,
	/// not nullable 
	pub ad_table_id:f64,
	/// not nullable 
	pub ad_window_id:f64,
	pub commitwarning:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub displaylogic:Option<String>,
	/// defaults to: 'D'::character varying
	/// not nullable 
	pub entitytype:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub hastree:String,
	pub help:Option<String>,
	pub importfields:Option<String>,
	pub included_tab_id:Option<f64>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isadvancedtab:String,
	/// defaults to: 'N'::bpchar
	pub isinfotab:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isinsertrecord:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isreadonly:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub issinglerow:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub issorttab:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub istranslationtab:String,
	/// not nullable 
	pub name:String,
	pub orderbyclause:Option<String>,
	/// defaults to: NULL::numeric
	pub parent_column_id:Option<f64>,
	pub processing:Option<String>,
	pub readonlylogic:Option<String>,
	/// not nullable 
	pub seqno:f64,
	/// not nullable 
	pub tablevel:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub whereclause:Option<String>,
	/// has one
	pub ad_client_id_ad_client:Option<AdClient>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub ad_table_id_ad_table:Option<AdTable>,
	/// has one
	pub ad_window_id_ad_window:Option<AdWindow>,
	/// has one
	pub ad_column_id_ad_column:Option<AdColumn>,
	/// has one
	pub ad_process_id_ad_process:Option<AdProcess>,
	/// has one
	pub ad_image_id_ad_image:Option<AdImage>,
	/// has one
	pub ad_columnsortorder_id_ad_column:Option<AdColumn>,
	/// has one
	pub ad_columnsortyesno_id_ad_column:Option<AdColumn>,
	/// has one
	pub entitytype_ad_entitytype:Option<AdEntitytype>,
	/// has one, self referential
	pub included_tab_id_ad_tab:Option<Box<AdTab>>,
	/// has many
	pub ad_field:Option<Vec<AdField>>,
	/// has many
	pub ad_tab:Option<Vec<AdTab>>,
	/// has many
	pub ad_tab_trl:Option<Vec<AdTabTrl>>,
	/// has many
	pub ad_userdef_tab:Option<Vec<AdUserdefTab>>,
	/// has many
	pub ad_userquery:Option<Vec<AdUserquery>>,
	/// has many
	pub asp_clientexception:Option<Vec<AspClientexception>>,
	/// has many
	pub asp_tab:Option<Vec<AspTab>>,
}

pub struct AdTabTrl {
	/// primary
	/// not nullable 
	pub ad_tab_id:f64,
	/// primary
	/// not nullable 
	pub ad_language:String,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub commitwarning:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub istranslated:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_tab_id_ad_tab:Option<AdTab>,
	/// has one
	pub ad_language_ad_language:Option<AdLanguage>,
}

pub struct AdTable {
	/// primary
	/// not nullable 
	pub ad_table_id:f64,
	/// not nullable 
	pub accesslevel:String,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_val_rule_id:Option<f64>,
	pub ad_window_id:Option<f64>,
	/// defaults to: NULL::numeric
	pub attribute_id:Option<f64>,
	pub copycolumnsfromtable:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'D'::character varying
	/// not nullable 
	pub entitytype:String,
	pub help:Option<String>,
	pub importtable:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub ischangelog:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isdeleteable:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub ishighvolume:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub issecurityenabled:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isview:String,
	pub loadseq:Option<f64>,
	/// not nullable 
	pub name:String,
	/// defaults to: NULL::numeric
	pub parent_id:Option<f64>,
	pub po_window_id:Option<f64>,
	/// defaults to: 'L'::bpchar
	/// not nullable 
	pub replicationtype:String,
	/// not nullable 
	pub tablename:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_client_id_ad_client:Option<AdClient>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub entitytype_ad_entitytype:Option<AdEntitytype>,
	/// has one
	pub ad_window_id_ad_window:Option<AdWindow>,
	/// has one
	pub ad_val_rule_id_ad_val_rule:Option<AdValRule>,
	/// has one
	pub po_window_id_ad_window:Option<AdWindow>,
	/// has many
	pub ad_accesslog:Option<Vec<AdAccesslog>>,
	/// has many
	pub ad_alertrule:Option<Vec<AdAlertrule>>,
	/// has many
	pub ad_archive:Option<Vec<AdArchive>>,
	/// has many
	pub ad_attachment:Option<Vec<AdAttachment>>,
	/// has many
	pub ad_attribute:Option<Vec<AdAttribute>>,
	/// has many
	pub ad_changelog:Option<Vec<AdChangelog>>,
	/// has many
	pub ad_clientshare:Option<Vec<AdClientshare>>,
	/// has many
	pub ad_column:Option<Vec<AdColumn>>,
	/// has many
	pub ad_column_access:Option<Vec<AdColumnAccess>>,
	/// has many
	pub ad_impformat:Option<Vec<AdImpformat>>,
	/// has many
	pub ad_infowindow:Option<Vec<AdInfowindow>>,
	/// has many
	pub ad_note:Option<Vec<AdNote>>,
	/// has many
	pub ad_package_exp_common:Option<Vec<AdPackageExpCommon>>,
	/// has many
	pub ad_package_exp_detail:Option<Vec<AdPackageExpDetail>>,
	/// has many
	pub ad_printformat:Option<Vec<AdPrintformat>>,
	/// has many
	pub ad_printlabel:Option<Vec<AdPrintlabel>>,
	/// has many
	pub ad_private_access:Option<Vec<AdPrivateAccess>>,
	/// has many
	pub ad_record_access:Option<Vec<AdRecordAccess>>,
	/// has many
	pub ad_ref_table:Option<Vec<AdRefTable>>,
	/// has many
	pub ad_replicationdocument:Option<Vec<AdReplicationdocument>>,
	/// has many
	pub ad_replicationtable:Option<Vec<AdReplicationtable>>,
	/// has many
	pub ad_reportview:Option<Vec<AdReportview>>,
	/// has many
	pub ad_sequence_audit:Option<Vec<AdSequenceAudit>>,
	/// has many
	pub ad_tab:Option<Vec<AdTab>>,
	/// has many
	pub ad_table_access:Option<Vec<AdTableAccess>>,
	/// has many
	pub ad_table_scriptvalidator:Option<Vec<AdTableScriptvalidator>>,
	/// has many
	pub ad_table_trl:Option<Vec<AdTableTrl>>,
	/// has many
	pub ad_userquery:Option<Vec<AdUserquery>>,
	/// has many
	pub ad_wf_activity:Option<Vec<AdWfActivity>>,
	/// has many
	pub ad_wf_eventaudit:Option<Vec<AdWfEventaudit>>,
	/// has many
	pub ad_wf_process:Option<Vec<AdWfProcess>>,
	/// has many
	pub ad_workflow:Option<Vec<AdWorkflow>>,
	/// has many
	pub c_acctprocessor:Option<Vec<CAcctprocessor>>,
	/// has many
	pub cm_chat:Option<Vec<CmChat>>,
	/// has many
	pub cm_chattype:Option<Vec<CmChattype>>,
	/// has many
	pub cm_templatetable:Option<Vec<CmTemplatetable>>,
	/// has many
	pub cm_wikitoken:Option<Vec<CmWikitoken>>,
	/// has many
	pub exp_format:Option<Vec<ExpFormat>>,
	/// has many
	pub fact_acct:Option<Vec<FactAcct>>,
	/// has many
	pub k_index:Option<Vec<KIndex>>,
	/// has many
	pub m_attributesetexclude:Option<Vec<MAttributesetexclude>>,
	/// has many
	pub m_lotctlexclude:Option<Vec<MLotctlexclude>>,
	/// has many
	pub m_sernoctlexclude:Option<Vec<MSernoctlexclude>>,
	/// has many
	pub pa_measurecalc:Option<Vec<PaMeasurecalc>>,
	/// has many
	pub pa_sla_measure:Option<Vec<PaSlaMeasure>>,
	/// has many
	pub pp_order_workflow:Option<Vec<PpOrderWorkflow>>,
	/// has many
	pub r_request:Option<Vec<RRequest>>,
}

pub struct AdTableAccess {
	/// primary
	/// not nullable 
	pub ad_role_id:f64,
	/// primary
	/// not nullable 
	pub ad_table_id:f64,
	/// primary
	/// defaults to: 'G'::bpchar
	/// not nullable 
	pub accesstyperule:String,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub iscanexport:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub iscanreport:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isexclude:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isreadonly:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_role_id_ad_role:Option<AdRole>,
	/// has one
	pub ad_table_id_ad_table:Option<AdTable>,
	/// has one
	pub ad_client_id_ad_client:Option<AdClient>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
}

pub struct AdTableScriptvalidator {
	/// primary
	/// not nullable 
	pub ad_table_scriptvalidator_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_rule_id:f64,
	/// not nullable 
	pub ad_table_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub eventmodelvalidator:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 0
	/// not nullable 
	pub seqno:f64,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_rule_id_ad_rule:Option<AdRule>,
	/// has one
	pub ad_table_id_ad_table:Option<AdTable>,
}

pub struct AdTableTrl {
	/// primary
	/// not nullable 
	pub ad_table_id:f64,
	/// primary
	/// not nullable 
	pub ad_language:String,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub istranslated:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_table_id_ad_table:Option<AdTable>,
	/// has one
	pub ad_language_ad_language:Option<AdLanguage>,
}

pub struct AdTask {
	/// primary
	/// not nullable 
	pub ad_task_id:f64,
	/// not nullable 
	pub accesslevel:String,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'D'::character varying
	/// not nullable 
	pub entitytype:String,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isserverprocess:String,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub os_command:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_client_id_ad_client:Option<AdClient>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub entitytype_ad_entitytype:Option<AdEntitytype>,
	/// has many
	pub ad_menu:Option<Vec<AdMenu>>,
	/// has many
	pub ad_task_access:Option<Vec<AdTaskAccess>>,
	/// has many
	pub ad_task_trl:Option<Vec<AdTaskTrl>>,
	/// has many
	pub ad_taskinstance:Option<Vec<AdTaskinstance>>,
	/// has many
	pub ad_wf_node:Option<Vec<AdWfNode>>,
	/// has many
	pub ad_workbenchwindow:Option<Vec<AdWorkbenchwindow>>,
	/// has many
	pub asp_clientexception:Option<Vec<AspClientexception>>,
	/// has many
	pub asp_task:Option<Vec<AspTask>>,
	/// has many
	pub pp_order_node:Option<Vec<PpOrderNode>>,
}

pub struct AdTaskAccess {
	/// primary
	/// not nullable 
	pub ad_task_id:f64,
	/// primary
	/// not nullable 
	pub ad_role_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isreadwrite:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_task_id_ad_task:Option<AdTask>,
	/// has one
	pub ad_role_id_ad_role:Option<AdRole>,
	/// has one
	pub ad_client_id_ad_client:Option<AdClient>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
}

pub struct AdTaskTrl {
	/// primary
	/// not nullable 
	pub ad_task_id:f64,
	/// primary
	/// not nullable 
	pub ad_language:String,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub istranslated:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_task_id_ad_task:Option<AdTask>,
	/// has one
	pub ad_language_ad_language:Option<AdLanguage>,
}

pub struct AdTaskinstance {
	/// primary
	/// not nullable 
	pub ad_taskinstance_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_task_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	pub createdby:Option<f64>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	pub updatedby:Option<f64>,
	/// has one
	pub ad_client_id_ad_client:Option<AdClient>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub ad_task_id_ad_task:Option<AdTask>,
}

pub struct AdTree {
	/// primary
	/// not nullable 
	pub ad_tree_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isallnodes:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isdefault:String,
	/// not nullable 
	pub name:String,
	pub processing:Option<String>,
	/// not nullable 
	pub treetype:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has many
	pub ad_clientinfo:Option<Vec<AdClientinfo>>,
	/// has many
	pub ad_role:Option<Vec<AdRole>>,
	/// has many
	pub ad_treebar:Option<Vec<AdTreebar>>,
	/// has many
	pub ad_treenode:Option<Vec<AdTreenode>>,
	/// has many
	pub ad_treenodebp:Option<Vec<AdTreenodebp>>,
	/// has many
	pub ad_treenodecmc:Option<Vec<AdTreenodecmc>>,
	/// has many
	pub ad_treenodecmm:Option<Vec<AdTreenodecmm>>,
	/// has many
	pub ad_treenodecms:Option<Vec<AdTreenodecms>>,
	/// has many
	pub ad_treenodecmt:Option<Vec<AdTreenodecmt>>,
	/// has many
	pub ad_treenodemm:Option<Vec<AdTreenodemm>>,
	/// has many
	pub ad_treenodepr:Option<Vec<AdTreenodepr>>,
	/// has many
	pub ad_treenodeu1:Option<Vec<AdTreenodeu1>>,
	/// has many
	pub ad_treenodeu2:Option<Vec<AdTreenodeu2>>,
	/// has many
	pub ad_treenodeu3:Option<Vec<AdTreenodeu3>>,
	/// has many
	pub ad_treenodeu4:Option<Vec<AdTreenodeu4>>,
	/// has many
	pub c_element:Option<Vec<CElement>>,
	/// has many
	pub cm_webproject:Option<Vec<CmWebproject>>,
	/// has many
	pub pa_hierarchy:Option<Vec<PaHierarchy>>,
}

pub struct AdTreebar {
	/// primary
	/// not nullable 
	pub ad_tree_id:f64,
	/// primary
	/// not nullable 
	pub ad_user_id:f64,
	/// primary
	/// not nullable 
	pub node_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_tree_id_ad_tree:Option<AdTree>,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
}

pub struct AdTreenode {
	/// primary
	/// not nullable 
	pub ad_tree_id:f64,
	/// primary
	/// not nullable 
	pub node_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub parent_id:Option<f64>,
	pub seqno:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_tree_id_ad_tree:Option<AdTree>,
}

pub struct AdTreenodebp {
	/// primary
	/// not nullable 
	pub ad_tree_id:f64,
	/// primary
	/// not nullable 
	pub node_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub parent_id:Option<f64>,
	pub seqno:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_tree_id_ad_tree:Option<AdTree>,
}

pub struct AdTreenodecmc {
	/// primary
	/// not nullable 
	pub ad_tree_id:f64,
	/// primary
	/// not nullable 
	pub node_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub parent_id:f64,
	/// defaults to: 0
	/// not nullable 
	pub seqno:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_tree_id_ad_tree:Option<AdTree>,
}

pub struct AdTreenodecmm {
	/// primary
	/// not nullable 
	pub ad_tree_id:f64,
	/// primary
	/// not nullable 
	pub node_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub parent_id:f64,
	/// defaults to: 0
	/// not nullable 
	pub seqno:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_tree_id_ad_tree:Option<AdTree>,
}

pub struct AdTreenodecms {
	/// primary
	/// not nullable 
	pub ad_tree_id:f64,
	/// primary
	/// not nullable 
	pub node_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub parent_id:f64,
	/// defaults to: 0
	/// not nullable 
	pub seqno:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_tree_id_ad_tree:Option<AdTree>,
}

pub struct AdTreenodecmt {
	/// primary
	/// not nullable 
	pub ad_tree_id:f64,
	/// primary
	/// not nullable 
	pub node_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub parent_id:f64,
	/// defaults to: 0
	/// not nullable 
	pub seqno:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_tree_id_ad_tree:Option<AdTree>,
}

pub struct AdTreenodemm {
	/// primary
	/// not nullable 
	pub ad_tree_id:f64,
	/// primary
	/// not nullable 
	pub node_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub parent_id:Option<f64>,
	pub seqno:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_tree_id_ad_tree:Option<AdTree>,
}

pub struct AdTreenodepr {
	/// primary
	/// not nullable 
	pub ad_tree_id:f64,
	/// primary
	/// not nullable 
	pub node_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub parent_id:Option<f64>,
	pub seqno:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_tree_id_ad_tree:Option<AdTree>,
}

pub struct AdTreenodeu1 {
	/// primary
	/// not nullable 
	pub ad_tree_id:f64,
	/// primary
	/// not nullable 
	pub node_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub parent_id:f64,
	/// defaults to: 0
	/// not nullable 
	pub seqno:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_tree_id_ad_tree:Option<AdTree>,
}

pub struct AdTreenodeu2 {
	/// primary
	/// not nullable 
	pub ad_tree_id:f64,
	/// primary
	/// not nullable 
	pub node_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub parent_id:f64,
	/// defaults to: 0
	/// not nullable 
	pub seqno:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_tree_id_ad_tree:Option<AdTree>,
}

pub struct AdTreenodeu3 {
	/// primary
	/// not nullable 
	pub ad_tree_id:f64,
	/// primary
	/// not nullable 
	pub node_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub parent_id:f64,
	/// defaults to: 0
	/// not nullable 
	pub seqno:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_tree_id_ad_tree:Option<AdTree>,
}

pub struct AdTreenodeu4 {
	/// primary
	/// not nullable 
	pub ad_tree_id:f64,
	/// primary
	/// not nullable 
	pub node_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub parent_id:f64,
	/// defaults to: 0
	/// not nullable 
	pub seqno:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_tree_id_ad_tree:Option<AdTree>,
}

pub struct AdUser {
	/// primary
	/// not nullable 
	pub ad_user_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_orgtrx_id:Option<f64>,
	pub birthday:Option<NaiveDateTime>,
	pub c_bpartner_id:Option<f64>,
	pub c_bpartner_location_id:Option<f64>,
	pub c_greeting_id:Option<f64>,
	pub c_job_id:Option<f64>,
	pub comments:Option<String>,
	pub connectionprofile:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub email:Option<String>,
	pub emailuser:Option<String>,
	pub emailuserpw:Option<String>,
	pub emailverify:Option<String>,
	pub emailverifydate:Option<NaiveDateTime>,
	pub fax:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isfullbpaccess:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isinpayroll:String,
	pub lastcontact:Option<NaiveDateTime>,
	pub lastresult:Option<String>,
	pub ldapuser:Option<String>,
	/// not nullable 
	pub name:String,
	/// defaults to: 'E'::bpchar
	/// not nullable 
	pub notificationtype:String,
	pub password:Option<String>,
	pub phone:Option<String>,
	pub phone2:Option<String>,
	pub processing:Option<String>,
	pub supervisor_id:Option<f64>,
	pub title:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub userpin:Option<String>,
	pub value:Option<String>,
	/// has one
	pub ad_client_id_ad_client:Option<AdClient>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one, self referential
	pub supervisor_id_ad_user:Option<Box<AdUser>>,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub c_bpartner_location_id_c_bpartner_location:Option<CBpartnerLocation>,
	/// has one
	pub c_greeting_id_c_greeting:Option<CGreeting>,
	/// has one
	pub ad_orgtrx_id_ad_org:Option<AdOrg>,
	/// has one
	pub c_job_id_c_job:Option<CJob>,
	/// has one, extension table
	pub b_buyer:Option<Box<BBuyer>>,
	/// has one, extension table
	pub b_seller:Option<Box<BSeller>>,
	/// has many
	pub a_asset:Option<Vec<AAsset>>,
	/// has many
	pub a_asset_change:Option<Vec<AAssetChange>>,
	/// has many
	pub a_asset_delivery:Option<Vec<AAssetDelivery>>,
	/// has many
	pub a_registration:Option<Vec<ARegistration>>,
	/// has many
	pub ad_alertprocessor:Option<Vec<AdAlertprocessor>>,
	/// has many
	pub ad_alertrecipient:Option<Vec<AdAlertrecipient>>,
	/// has many
	pub ad_attachmentnote:Option<Vec<AdAttachmentnote>>,
	/// has many
	pub ad_ldapaccess:Option<Vec<AdLdapaccess>>,
	/// has many
	pub ad_ldapprocessor:Option<Vec<AdLdapprocessor>>,
	/// has many
	pub ad_note:Option<Vec<AdNote>>,
	/// has many
	pub ad_orginfo:Option<Vec<AdOrginfo>>,
	/// has many
	pub ad_pinstance:Option<Vec<AdPinstance>>,
	/// has many
	pub ad_preference:Option<Vec<AdPreference>>,
	/// has many
	pub ad_private_access:Option<Vec<AdPrivateAccess>>,
	/// has many
	pub ad_role:Option<Vec<AdRole>>,
	/// has many
	pub ad_scheduler:Option<Vec<AdScheduler>>,
	/// has many
	pub ad_schedulerrecipient:Option<Vec<AdSchedulerrecipient>>,
	/// has many
	pub ad_treebar:Option<Vec<AdTreebar>>,
	/// has many
	pub ad_user:Option<Vec<AdUser>>,
	/// has many
	pub ad_user_orgaccess:Option<Vec<AdUserOrgaccess>>,
	/// has many
	pub ad_user_roles:Option<Vec<AdUserRoles>>,
	/// has many
	pub ad_user_substitute:Option<Vec<AdUserSubstitute>>,
	/// has many
	pub ad_userbpaccess:Option<Vec<AdUserbpaccess>>,
	/// has many
	pub ad_userdef_win:Option<Vec<AdUserdefWin>>,
	/// has many
	pub ad_usermail:Option<Vec<AdUsermail>>,
	/// has many
	pub ad_userquery:Option<Vec<AdUserquery>>,
	/// has many
	pub ad_wf_activity:Option<Vec<AdWfActivity>>,
	/// has many
	pub ad_wf_eventaudit:Option<Vec<AdWfEventaudit>>,
	/// has many
	pub ad_wf_process:Option<Vec<AdWfProcess>>,
	/// has many
	pub ad_wf_responsible:Option<Vec<AdWfResponsible>>,
	/// has many
	pub ad_workflowprocessor:Option<Vec<AdWorkflowprocessor>>,
	/// has many
	pub b_bid:Option<Vec<BBid>>,
	/// has many
	pub b_bidcomment:Option<Vec<BBidcomment>>,
	/// has many
	pub b_buyerfunds:Option<Vec<BBuyerfunds>>,
	/// has many
	pub b_offer:Option<Vec<BOffer>>,
	/// has many
	pub b_sellerfunds:Option<Vec<BSellerfunds>>,
	/// has many
	pub c_acctprocessor:Option<Vec<CAcctprocessor>>,
	/// has many
	pub c_bp_bankaccount:Option<Vec<CBpBankaccount>>,
	/// has many
	pub c_bpartner:Option<Vec<CBpartner>>,
	/// has many
	pub c_dunningrunentry:Option<Vec<CDunningrunentry>>,
	/// has many
	pub c_invoice:Option<Vec<CInvoice>>,
	/// has many
	pub c_invoicebatch:Option<Vec<CInvoicebatch>>,
	/// has many
	pub c_invoicebatchline:Option<Vec<CInvoicebatchline>>,
	/// has many
	pub c_jobassignment:Option<Vec<CJobassignment>>,
	/// has many
	pub c_order:Option<Vec<COrder>>,
	/// has many
	pub c_orgassignment:Option<Vec<COrgassignment>>,
	/// has many
	pub c_pos:Option<Vec<CPos>>,
	/// has many
	pub c_project:Option<Vec<CProject>>,
	/// has many
	pub c_rfq:Option<Vec<CRfq>>,
	/// has many
	pub c_rfq_topicsubscriber:Option<Vec<CRfqTopicsubscriber>>,
	/// has many
	pub c_rfqresponse:Option<Vec<CRfqresponse>>,
	/// has many
	pub c_salesregion:Option<Vec<CSalesregion>>,
	/// has many
	pub c_userremuneration:Option<Vec<CUserremuneration>>,
	/// has many
	pub cm_chatentry:Option<Vec<CmChatentry>>,
	/// has many
	pub cm_chattypeupdate:Option<Vec<CmChattypeupdate>>,
	/// has many
	pub cm_chatupdate:Option<Vec<CmChatupdate>>,
	/// has many
	pub cm_webaccesslog:Option<Vec<CmWebaccesslog>>,
	/// has many
	pub dd_order:Option<Vec<DdOrder>>,
	/// has many
	pub hr_job:Option<Vec<HrJob>>,
	/// has many
	pub i_bpartner:Option<Vec<IBpartner>>,
	/// has many
	pub i_invoice:Option<Vec<IInvoice>>,
	/// has many
	pub i_order:Option<Vec<IOrder>>,
	/// has many
	pub m_inout:Option<Vec<MInout>>,
	/// has many
	pub m_movement:Option<Vec<MMovement>>,
	/// has many
	pub m_product:Option<Vec<MProduct>>,
	/// has many
	pub m_requisition:Option<Vec<MRequisition>>,
	/// has many
	pub m_rma:Option<Vec<MRma>>,
	/// has many
	pub pa_goal:Option<Vec<PaGoal>>,
	/// has many
	pub pp_cost_collector:Option<Vec<PpCostCollector>>,
	/// has many
	pub pp_mrp:Option<Vec<PpMrp>>,
	/// has many
	pub pp_order:Option<Vec<PpOrder>>,
	/// has many
	pub pp_order_bomline:Option<Vec<PpOrderBomline>>,
	/// has many
	pub r_categoryupdates:Option<Vec<RCategoryupdates>>,
	/// has many
	pub r_contactinterest:Option<Vec<RContactinterest>>,
	/// has many
	pub r_groupupdates:Option<Vec<RGroupupdates>>,
	/// has many
	pub r_issueuser:Option<Vec<RIssueuser>>,
	/// has many
	pub r_request:Option<Vec<RRequest>>,
	/// has many
	pub r_requestaction:Option<Vec<RRequestaction>>,
	/// has many
	pub r_requestprocessor:Option<Vec<RRequestprocessor>>,
	/// has many
	pub r_requestprocessor_route:Option<Vec<RRequestprocessorRoute>>,
	/// has many
	pub r_requesttypeupdates:Option<Vec<RRequesttypeupdates>>,
	/// has many
	pub r_requestupdates:Option<Vec<RRequestupdates>>,
	/// has many
	pub s_resource:Option<Vec<SResource>>,
	/// has many
	pub w_advertisement:Option<Vec<WAdvertisement>>,
	/// has many
	pub w_basket:Option<Vec<WBasket>>,
	/// has many
	pub w_click:Option<Vec<WClick>>,
	/// has many
	pub w_counter:Option<Vec<WCounter>>,
	/// has many
	pub w_store:Option<Vec<WStore>>,
}

pub struct AdUserOrgaccess {
	/// primary
	/// not nullable 
	pub ad_user_id:f64,
	/// primary
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isreadonly:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
}

pub struct AdUserRoles {
	/// primary
	/// not nullable 
	pub ad_user_id:f64,
	/// primary
	/// not nullable 
	pub ad_role_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
	/// has one
	pub ad_role_id_ad_role:Option<AdRole>,
	/// has one
	pub ad_client_id_ad_client:Option<AdClient>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
}

pub struct AdUserSubstitute {
	/// primary
	/// not nullable 
	pub ad_user_substitute_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_user_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub substitute_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub validfrom:Option<NaiveDateTime>,
	pub validto:Option<NaiveDateTime>,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
	/// has one
	pub substitute_id_ad_user:Option<AdUser>,
}

pub struct AdUserbpaccess {
	/// primary
	/// not nullable 
	pub ad_userbpaccess_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_user_id:f64,
	/// not nullable 
	pub bpaccesstype:String,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub docbasetype:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub r_requesttype_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
	/// has one
	pub r_requesttype_id_r_requesttype:Option<RRequesttype>,
}

pub struct AdUserdefField {
	/// primary
	/// not nullable 
	pub ad_userdef_field_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_field_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_userdef_tab_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub defaultvalue:String,
	pub description:Option<String>,
	/// defaults to: 0
	/// not nullable 
	pub displaylength:f64,
	/// not nullable 
	pub displaylogic:String,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isdisplayed:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isreadonly:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub issameline:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isupdateable:String,
	/// not nullable 
	pub name:String,
	/// defaults to: 0
	/// not nullable 
	pub seqno:f64,
	/// defaults to: 0
	/// not nullable 
	pub sortno:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_userdef_tab_id_ad_userdef_tab:Option<AdUserdefTab>,
	/// has one
	pub ad_field_id_ad_field:Option<AdField>,
}

pub struct AdUserdefTab {
	/// primary
	/// not nullable 
	pub ad_userdef_tab_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_tab_id:f64,
	/// not nullable 
	pub ad_userdef_win_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub ismultirowonly:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isreadonly:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub issinglerow:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_userdef_win_id_ad_userdef_win:Option<AdUserdefWin>,
	/// has one
	pub ad_tab_id_ad_tab:Option<AdTab>,
	/// has many
	pub ad_userdef_field:Option<Vec<AdUserdefField>>,
}

pub struct AdUserdefWin {
	/// primary
	/// not nullable 
	pub ad_userdef_win_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	pub ad_language:Option<String>,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_role_id:Option<f64>,
	pub ad_user_id:Option<f64>,
	/// not nullable 
	pub ad_window_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isdefault:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isreadonly:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isuserupdateable:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_role_id_ad_role:Option<AdRole>,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
	/// has one
	pub ad_window_id_ad_window:Option<AdWindow>,
	/// has one
	pub ad_language_ad_language:Option<AdLanguage>,
	/// has many
	pub ad_userdef_tab:Option<Vec<AdUserdefTab>>,
}

pub struct AdUsermail {
	/// primary
	/// not nullable 
	pub ad_usermail_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_user_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub deliveryconfirmation:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub isdelivered:Option<String>,
	pub mailtext:Option<String>,
	pub messageid:Option<String>,
	pub r_mailtext_id:Option<f64>,
	pub subject:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub w_mailmsg_id:Option<f64>,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
	/// has one
	pub r_mailtext_id_r_mailtext:Option<RMailtext>,
	/// has one
	pub w_mailmsg_id_w_mailmsg:Option<WMailmsg>,
}

pub struct AdUserquery {
	/// primary
	/// not nullable 
	pub ad_userquery_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_tab_id:Option<f64>,
	/// not nullable 
	pub ad_table_id:f64,
	pub ad_user_id:Option<f64>,
	pub code:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
	/// has one
	pub ad_table_id_ad_table:Option<AdTable>,
	/// has one
	pub ad_tab_id_ad_tab:Option<AdTab>,
}

pub struct AdValRule {
	/// primary
	/// not nullable 
	pub ad_val_rule_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub code:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'D'::character varying
	/// not nullable 
	pub entitytype:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	pub type_:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_client_id_ad_client:Option<AdClient>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub entitytype_ad_entitytype:Option<AdEntitytype>,
	/// has many
	pub ad_attribute:Option<Vec<AdAttribute>>,
	/// has many
	pub ad_column:Option<Vec<AdColumn>>,
	/// has many
	pub ad_field:Option<Vec<AdField>>,
	/// has many
	pub ad_process_para:Option<Vec<AdProcessPara>>,
	/// has many
	pub ad_table:Option<Vec<AdTable>>,
}

pub struct AdWfActivity {
	/// primary
	/// not nullable 
	pub ad_wf_activity_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	pub ad_message_id:Option<f64>,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_table_id:f64,
	pub ad_user_id:Option<f64>,
	/// not nullable 
	pub ad_wf_node_id:f64,
	/// not nullable 
	pub ad_wf_process_id:f64,
	pub ad_wf_responsible_id:Option<f64>,
	/// not nullable 
	pub ad_workflow_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub datelastalert:Option<NaiveDateTime>,
	pub dynprioritystart:Option<f64>,
	pub endwaittime:Option<NaiveDateTime>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub priority:Option<f64>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	pub processing:Option<String>,
	/// not nullable 
	pub record_id:f64,
	pub textmsg:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub wfstate:String,
	/// has one
	pub ad_wf_process_id_ad_wf_process:Option<AdWfProcess>,
	/// has one
	pub ad_wf_node_id_ad_wf_node:Option<AdWfNode>,
	/// has one
	pub ad_wf_responsible_id_ad_wf_responsible:Option<AdWfResponsible>,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
	/// has one
	pub ad_message_id_ad_message:Option<AdMessage>,
	/// has one
	pub ad_workflow_id_ad_workflow:Option<AdWorkflow>,
	/// has one
	pub ad_table_id_ad_table:Option<AdTable>,
	/// has many
	pub ad_note:Option<Vec<AdNote>>,
	/// has many
	pub ad_wf_activityresult:Option<Vec<AdWfActivityresult>>,
}

pub struct AdWfActivityresult {
	/// primary
	/// not nullable 
	pub ad_wf_activityresult_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_wf_activity_id:f64,
	/// not nullable 
	pub attributename:String,
	pub attributevalue:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_wf_activity_id_ad_wf_activity:Option<AdWfActivity>,
}

pub struct AdWfBlock {
	/// primary
	/// not nullable 
	pub ad_wf_block_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_workflow_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_workflow_id_ad_workflow:Option<AdWorkflow>,
	/// has many
	pub ad_wf_node:Option<Vec<AdWfNode>>,
	/// has many
	pub pp_order_node:Option<Vec<PpOrderNode>>,
}

pub struct AdWfEventaudit {
	/// primary
	/// not nullable 
	pub ad_wf_eventaudit_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_table_id:f64,
	pub ad_user_id:Option<f64>,
	/// not nullable 
	pub ad_wf_node_id:f64,
	/// not nullable 
	pub ad_wf_process_id:f64,
	/// not nullable 
	pub ad_wf_responsible_id:f64,
	pub attributename:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// not nullable 
	pub elapsedtimems:f64,
	/// not nullable 
	pub eventtype:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub newvalue:Option<String>,
	pub oldvalue:Option<String>,
	/// not nullable 
	pub record_id:f64,
	pub textmsg:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub wfstate:String,
	/// has one
	pub ad_wf_process_id_ad_wf_process:Option<AdWfProcess>,
	/// has one
	pub ad_wf_node_id_ad_wf_node:Option<AdWfNode>,
	/// has one
	pub ad_table_id_ad_table:Option<AdTable>,
	/// has one
	pub ad_wf_responsible_id_ad_wf_responsible:Option<AdWfResponsible>,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
}

pub struct AdWfNextcondition {
	/// primary
	/// not nullable 
	pub ad_wf_nextcondition_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_column_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_wf_nodenext_id:f64,
	/// not nullable 
	pub andor:String,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'D'::character varying
	/// not nullable 
	pub entitytype:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub operation:String,
	/// not nullable 
	pub seqno:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
	pub value2:Option<String>,
	/// has one
	pub ad_wf_nodenext_id_ad_wf_nodenext:Option<AdWfNodenext>,
	/// has one
	pub entitytype_ad_entitytype:Option<AdEntitytype>,
	/// has one
	pub ad_column_id_ad_column:Option<AdColumn>,
}

pub struct AdWfNode {
	/// primary
	/// not nullable 
	pub ad_wf_node_id:f64,
	/// not nullable 
	pub action:String,
	/// not nullable 
	pub ad_client_id:f64,
	pub ad_column_id:Option<f64>,
	pub ad_form_id:Option<f64>,
	pub ad_image_id:Option<f64>,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_process_id:Option<f64>,
	pub ad_task_id:Option<f64>,
	pub ad_wf_block_id:Option<f64>,
	pub ad_wf_responsible_id:Option<f64>,
	pub ad_window_id:Option<f64>,
	/// not nullable 
	pub ad_workflow_id:f64,
	pub attributename:Option<String>,
	pub attributevalue:Option<String>,
	pub c_bpartner_id:Option<f64>,
	/// defaults to: 0
	/// not nullable 
	pub cost:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub docaction:Option<String>,
	/// defaults to: 0
	/// not nullable 
	pub duration:f64,
	pub dynprioritychange:Option<f64>,
	pub dynpriorityunit:Option<String>,
	pub email:Option<String>,
	pub emailrecipient:Option<String>,
	/// defaults to: 'U'::character varying
	/// not nullable 
	pub entitytype:String,
	pub finishmode:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub iscentrallymaintained:String,
	/// defaults to: 'N'::bpchar
	pub ismilestone:Option<String>,
	/// defaults to: 'N'::bpchar
	pub issubcontracting:Option<String>,
	/// not nullable 
	pub joinelement:String,
	/// defaults to: 0
	/// not nullable 
	pub limit:f64,
	pub movingtime:Option<f64>,
	/// not nullable 
	pub name:String,
	pub overlapunits:Option<f64>,
	pub priority:Option<f64>,
	pub queuingtime:Option<f64>,
	pub r_mailtext_id:Option<f64>,
	pub s_resource_id:Option<f64>,
	pub setuptime:Option<f64>,
	/// not nullable 
	pub splitelement:String,
	pub startmode:Option<String>,
	pub subflowexecution:Option<String>,
	/// defaults to: (0)::numeric
	pub unitscycles:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub validfrom:Option<NaiveDateTime>,
	pub validto:Option<NaiveDateTime>,
	/// not nullable 
	pub value:String,
	/// defaults to: 0
	/// not nullable 
	pub waitingtime:f64,
	pub waittime:Option<f64>,
	pub workflow_id:Option<f64>,
	pub workingtime:Option<f64>,
	/// defaults to: 0
	/// not nullable 
	pub xposition:f64,
	/// defaults to: (100)::numeric
	pub yield_:Option<f64>,
	/// defaults to: 0
	/// not nullable 
	pub yposition:f64,
	/// has one
	pub ad_client_id_ad_client:Option<AdClient>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub ad_workflow_id_ad_workflow:Option<AdWorkflow>,
	/// has one
	pub ad_window_id_ad_window:Option<AdWindow>,
	/// has one
	pub workflow_id_ad_workflow:Option<AdWorkflow>,
	/// has one
	pub ad_task_id_ad_task:Option<AdTask>,
	/// has one
	pub ad_process_id_ad_process:Option<AdProcess>,
	/// has one
	pub ad_form_id_ad_form:Option<AdForm>,
	/// has one
	pub entitytype_ad_entitytype:Option<AdEntitytype>,
	/// has one
	pub ad_wf_block_id_ad_wf_block:Option<AdWfBlock>,
	/// has one
	pub ad_wf_responsible_id_ad_wf_responsible:Option<AdWfResponsible>,
	/// has one
	pub ad_image_id_ad_image:Option<AdImage>,
	/// has one
	pub ad_column_id_ad_column:Option<AdColumn>,
	/// has one
	pub r_mailtext_id_r_mailtext:Option<RMailtext>,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub s_resource_id_s_resource:Option<SResource>,
	/// has many
	pub ad_wf_activity:Option<Vec<AdWfActivity>>,
	/// has many
	pub ad_wf_eventaudit:Option<Vec<AdWfEventaudit>>,
	/// has many
	pub ad_wf_node_para:Option<Vec<AdWfNodePara>>,
	/// has many
	pub ad_wf_node_trl:Option<Vec<AdWfNodeTrl>>,
	/// has many
	pub ad_wf_nodenext:Option<Vec<AdWfNodenext>>,
	/// has many
	pub ad_workflow:Option<Vec<AdWorkflow>>,
	/// has many
	pub asp_clientexception:Option<Vec<AspClientexception>>,
	/// has many
	pub pp_order_node:Option<Vec<PpOrderNode>>,
	/// has many
	pub pp_order_nodenext:Option<Vec<PpOrderNodenext>>,
	/// has many
	pub pp_order_workflow:Option<Vec<PpOrderWorkflow>>,
	/// has many
	pub pp_wf_node_asset:Option<Vec<PpWfNodeAsset>>,
	/// has many
	pub pp_wf_node_product:Option<Vec<PpWfNodeProduct>>,
}

pub struct AdWfNodePara {
	/// primary
	/// not nullable 
	pub ad_wf_node_para_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_process_para_id:Option<f64>,
	/// not nullable 
	pub ad_wf_node_id:f64,
	pub attributename:Option<String>,
	pub attributevalue:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'D'::character varying
	/// not nullable 
	pub entitytype:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_wf_node_id_ad_wf_node:Option<AdWfNode>,
	/// has one
	pub ad_process_para_id_ad_process_para:Option<AdProcessPara>,
	/// has one
	pub entitytype_ad_entitytype:Option<AdEntitytype>,
}

pub struct AdWfNodeTrl {
	/// primary
	/// not nullable 
	pub ad_language:String,
	/// primary
	/// not nullable 
	pub ad_wf_node_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub istranslated:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_language_ad_language:Option<AdLanguage>,
	/// has one
	pub ad_wf_node_id_ad_wf_node:Option<AdWfNode>,
}

pub struct AdWfNodenext {
	/// primary
	/// not nullable 
	pub ad_wf_nodenext_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_wf_next_id:f64,
	/// not nullable 
	pub ad_wf_node_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'D'::character varying
	/// not nullable 
	pub entitytype:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isstduserworkflow:String,
	/// not nullable 
	pub seqno:f64,
	pub transitioncode:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_client_id_ad_client:Option<AdClient>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub ad_wf_node_id_ad_wf_node:Option<AdWfNode>,
	/// has one
	pub ad_wf_next_id_ad_wf_node:Option<AdWfNode>,
	/// has one
	pub entitytype_ad_entitytype:Option<AdEntitytype>,
	/// has many
	pub ad_wf_nextcondition:Option<Vec<AdWfNextcondition>>,
}

pub struct AdWfProcess {
	/// primary
	/// not nullable 
	pub ad_wf_process_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	pub ad_message_id:Option<f64>,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_table_id:f64,
	pub ad_user_id:Option<f64>,
	/// not nullable 
	pub ad_wf_responsible_id:f64,
	/// not nullable 
	pub ad_workflow_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub priority:Option<f64>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	pub processing:Option<String>,
	/// not nullable 
	pub record_id:f64,
	pub textmsg:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub wfstate:String,
	/// has one, self referential
	pub ad_wf_process_id_ad_wf_process:Option<Box<AdWfProcess>>,
	/// has one
	pub ad_client_id_ad_client:Option<AdClient>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub ad_workflow_id_ad_workflow:Option<AdWorkflow>,
	/// has one
	pub ad_wf_responsible_id_ad_wf_responsible:Option<AdWfResponsible>,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
	/// has one
	pub ad_message_id_ad_message:Option<AdMessage>,
	/// has one
	pub ad_table_id_ad_table:Option<AdTable>,
	/// has many
	pub ad_wf_activity:Option<Vec<AdWfActivity>>,
	/// has many
	pub ad_wf_eventaudit:Option<Vec<AdWfEventaudit>>,
	/// has many
	pub ad_wf_process:Option<Vec<AdWfProcess>>,
	/// has many
	pub ad_wf_processdata:Option<Vec<AdWfProcessdata>>,
}

pub struct AdWfProcessdata {
	/// primary
	/// not nullable 
	pub ad_wf_processdata_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_wf_process_id:f64,
	/// not nullable 
	pub attributename:String,
	pub attributevalue:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_wf_process_id_ad_wf_process:Option<AdWfProcess>,
}

pub struct AdWfResponsible {
	/// primary
	/// not nullable 
	pub ad_wf_responsible_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_role_id:Option<f64>,
	pub ad_user_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'D'::character varying
	/// not nullable 
	pub entitytype:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub responsibletype:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
	/// has one
	pub ad_role_id_ad_role:Option<AdRole>,
	/// has one
	pub entitytype_ad_entitytype:Option<AdEntitytype>,
	/// has many
	pub ad_wf_activity:Option<Vec<AdWfActivity>>,
	/// has many
	pub ad_wf_eventaudit:Option<Vec<AdWfEventaudit>>,
	/// has many
	pub ad_wf_node:Option<Vec<AdWfNode>>,
	/// has many
	pub ad_wf_process:Option<Vec<AdWfProcess>>,
	/// has many
	pub ad_workflow:Option<Vec<AdWorkflow>>,
	/// has many
	pub pp_order_node:Option<Vec<PpOrderNode>>,
	/// has many
	pub pp_order_workflow:Option<Vec<PpOrderWorkflow>>,
}

pub struct AdWindow {
	/// primary
	/// not nullable 
	pub ad_window_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	pub ad_color_id:Option<f64>,
	pub ad_image_id:Option<f64>,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'D'::character varying
	/// not nullable 
	pub entitytype:String,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isbetafunctionality:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isdefault:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub issotrx:String,
	/// not nullable 
	pub name:String,
	pub processing:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub windowtype:Option<String>,
	pub winheight:Option<f64>,
	pub winwidth:Option<f64>,
	/// has one
	pub ad_client_id_ad_client:Option<AdClient>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub entitytype_ad_entitytype:Option<AdEntitytype>,
	/// has one
	pub ad_image_id_ad_image:Option<AdImage>,
	/// has one
	pub ad_color_id_ad_color:Option<AdColor>,
	/// has many
	pub ad_issue:Option<Vec<AdIssue>>,
	/// has many
	pub ad_menu:Option<Vec<AdMenu>>,
	/// has many
	pub ad_package_exp_common:Option<Vec<AdPackageExpCommon>>,
	/// has many
	pub ad_package_exp_detail:Option<Vec<AdPackageExpDetail>>,
	/// has many
	pub ad_preference:Option<Vec<AdPreference>>,
	/// has many
	pub ad_ref_table:Option<Vec<AdRefTable>>,
	/// has many
	pub ad_tab:Option<Vec<AdTab>>,
	/// has many
	pub ad_table:Option<Vec<AdTable>>,
	/// has many
	pub ad_userdef_win:Option<Vec<AdUserdefWin>>,
	/// has many
	pub ad_wf_node:Option<Vec<AdWfNode>>,
	/// has many
	pub ad_window_access:Option<Vec<AdWindowAccess>>,
	/// has many
	pub ad_window_trl:Option<Vec<AdWindowTrl>>,
	/// has many
	pub ad_workbenchwindow:Option<Vec<AdWorkbenchwindow>>,
	/// has many
	pub asp_clientexception:Option<Vec<AspClientexception>>,
	/// has many
	pub asp_window:Option<Vec<AspWindow>>,
	/// has many
	pub pa_dashboardcontent:Option<Vec<PaDashboardcontent>>,
	/// has many
	pub pp_order_node:Option<Vec<PpOrderNode>>,
}

pub struct AdWindowAccess {
	/// primary
	/// not nullable 
	pub ad_window_id:f64,
	/// primary
	/// not nullable 
	pub ad_role_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: NULL::numeric
	pub ad_window_access_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isreadwrite:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_window_id_ad_window:Option<AdWindow>,
	/// has one
	pub ad_role_id_ad_role:Option<AdRole>,
	/// has one
	pub ad_client_id_ad_client:Option<AdClient>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
}

pub struct AdWindowTrl {
	/// primary
	/// not nullable 
	pub ad_window_id:f64,
	/// primary
	/// not nullable 
	pub ad_language:String,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub istranslated:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_window_id_ad_window:Option<AdWindow>,
	/// has one
	pub ad_language_ad_language:Option<AdLanguage>,
}

pub struct AdWorkbench {
	/// primary
	/// not nullable 
	pub ad_workbench_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	pub ad_color_id:Option<f64>,
	/// not nullable 
	pub ad_column_id:f64,
	pub ad_image_id:Option<f64>,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'D'::character varying
	/// not nullable 
	pub entitytype:String,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_column_id_ad_column:Option<AdColumn>,
	/// has one
	pub ad_image_id_ad_image:Option<AdImage>,
	/// has one
	pub ad_color_id_ad_color:Option<AdColor>,
	/// has one
	pub entitytype_ad_entitytype:Option<AdEntitytype>,
	/// has many
	pub ad_desktopworkbench:Option<Vec<AdDesktopworkbench>>,
	/// has many
	pub ad_menu:Option<Vec<AdMenu>>,
	/// has many
	pub ad_package_exp_common:Option<Vec<AdPackageExpCommon>>,
	/// has many
	pub ad_package_exp_detail:Option<Vec<AdPackageExpDetail>>,
	/// has many
	pub ad_workbench_trl:Option<Vec<AdWorkbenchTrl>>,
	/// has many
	pub ad_workbenchwindow:Option<Vec<AdWorkbenchwindow>>,
}

pub struct AdWorkbenchTrl {
	/// primary
	/// not nullable 
	pub ad_workbench_id:f64,
	/// primary
	/// not nullable 
	pub ad_language:String,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub istranslated:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_workbench_id_ad_workbench:Option<AdWorkbench>,
	/// has one
	pub ad_language_ad_language:Option<AdLanguage>,
}

pub struct AdWorkbenchwindow {
	/// primary
	/// not nullable 
	pub ad_workbenchwindow_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	pub ad_form_id:Option<f64>,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_process_id:Option<f64>,
	pub ad_task_id:Option<f64>,
	pub ad_window_id:Option<f64>,
	/// not nullable 
	pub ad_workbench_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'D'::character varying
	/// not nullable 
	pub entitytype:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isprimary:String,
	/// not nullable 
	pub seqno:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_workbench_id_ad_workbench:Option<AdWorkbench>,
	/// has one
	pub ad_window_id_ad_window:Option<AdWindow>,
	/// has one
	pub ad_form_id_ad_form:Option<AdForm>,
	/// has one
	pub ad_process_id_ad_process:Option<AdProcess>,
	/// has one
	pub ad_task_id_ad_task:Option<AdTask>,
	/// has one
	pub entitytype_ad_entitytype:Option<AdEntitytype>,
}

pub struct AdWorkflow {
	/// primary
	/// not nullable 
	pub ad_workflow_id:f64,
	/// not nullable 
	pub accesslevel:String,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_table_id:Option<f64>,
	pub ad_wf_node_id:Option<f64>,
	pub ad_wf_responsible_id:Option<f64>,
	pub ad_workflowprocessor_id:Option<f64>,
	/// defaults to: 'ComPiere'::character varying
	/// not nullable 
	pub author:String,
	/// not nullable 
	pub cost:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub documentno:Option<String>,
	pub docvaluelogic:Option<String>,
	/// defaults to: 0
	/// not nullable 
	pub duration:f64,
	pub durationunit:Option<String>,
	/// defaults to: 'D'::character varying
	/// not nullable 
	pub entitytype:String,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isbetafunctionality:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isdefault:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isvalid:String,
	pub limit:Option<f64>,
	pub movingtime:Option<f64>,
	/// not nullable 
	pub name:String,
	pub overlapunits:Option<f64>,
	pub priority:Option<f64>,
	pub processtype:Option<String>,
	/// not nullable 
	pub publishstatus:String,
	/// defaults to: (1)::numeric
	pub qtybatchsize:Option<f64>,
	pub queuingtime:Option<f64>,
	pub s_resource_id:Option<f64>,
	pub setuptime:Option<f64>,
	pub unitscycles:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub validateworkflow:Option<String>,
	pub validfrom:Option<NaiveDateTime>,
	pub validto:Option<NaiveDateTime>,
	/// not nullable 
	pub value:String,
	/// defaults to: 0
	/// not nullable 
	pub version:f64,
	/// defaults to: 0
	/// not nullable 
	pub waitingtime:f64,
	/// defaults to: 'G'::bpchar
	/// not nullable 
	pub workflowtype:String,
	/// defaults to: 0
	/// not nullable 
	pub workingtime:f64,
	/// defaults to: (100)::numeric
	pub yield_:Option<f64>,
	/// has one
	pub ad_client_id_ad_client:Option<AdClient>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub ad_wf_node_id_ad_wf_node:Option<AdWfNode>,
	/// has one
	pub entitytype_ad_entitytype:Option<AdEntitytype>,
	/// has one
	pub ad_wf_responsible_id_ad_wf_responsible:Option<AdWfResponsible>,
	/// has one
	pub ad_workflowprocessor_id_ad_workflowprocessor:Option<AdWorkflowprocessor>,
	/// has one
	pub ad_table_id_ad_table:Option<AdTable>,
	/// has one
	pub s_resource_id_s_resource:Option<SResource>,
	/// has many
	pub ad_menu:Option<Vec<AdMenu>>,
	/// has many
	pub ad_package_exp_common:Option<Vec<AdPackageExpCommon>>,
	/// has many
	pub ad_package_exp_detail:Option<Vec<AdPackageExpDetail>>,
	/// has many
	pub ad_process:Option<Vec<AdProcess>>,
	/// has many
	pub ad_wf_activity:Option<Vec<AdWfActivity>>,
	/// has many
	pub ad_wf_block:Option<Vec<AdWfBlock>>,
	/// has many
	pub ad_wf_node:Option<Vec<AdWfNode>>,
	/// has many
	pub ad_wf_process:Option<Vec<AdWfProcess>>,
	/// has many
	pub ad_workflow_access:Option<Vec<AdWorkflowAccess>>,
	/// has many
	pub ad_workflow_trl:Option<Vec<AdWorkflowTrl>>,
	/// has many
	pub asp_clientexception:Option<Vec<AspClientexception>>,
	/// has many
	pub asp_workflow:Option<Vec<AspWorkflow>>,
	/// has many
	pub hr_process:Option<Vec<HrProcess>>,
	/// has many
	pub pp_order:Option<Vec<PpOrder>>,
	/// has many
	pub pp_order_cost:Option<Vec<PpOrderCost>>,
	/// has many
	pub pp_order_node:Option<Vec<PpOrderNode>>,
	/// has many
	pub pp_order_workflow:Option<Vec<PpOrderWorkflow>>,
	/// has many
	pub qm_specification:Option<Vec<QmSpecification>>,
}

pub struct AdWorkflowAccess {
	/// primary
	/// not nullable 
	pub ad_workflow_id:f64,
	/// primary
	/// not nullable 
	pub ad_role_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isreadwrite:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_workflow_id_ad_workflow:Option<AdWorkflow>,
	/// has one
	pub ad_role_id_ad_role:Option<AdRole>,
	/// has one
	pub ad_client_id_ad_client:Option<AdClient>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
}

pub struct AdWorkflowTrl {
	/// primary
	/// not nullable 
	pub ad_workflow_id:f64,
	/// primary
	/// not nullable 
	pub ad_language:String,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub istranslated:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_workflow_id_ad_workflow:Option<AdWorkflow>,
	/// has one
	pub ad_language_ad_language:Option<AdLanguage>,
}

pub struct AdWorkflowprocessor {
	/// primary
	/// not nullable 
	pub ad_workflowprocessor_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub alertoverpriority:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub datelastrun:Option<NaiveDateTime>,
	pub datenextrun:Option<NaiveDateTime>,
	pub description:Option<String>,
	/// not nullable 
	pub frequency:f64,
	/// not nullable 
	pub frequencytype:String,
	/// defaults to: 0
	pub inactivityalertdays:Option<f64>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub keeplogdays:f64,
	/// not nullable 
	pub name:String,
	pub processing:Option<String>,
	/// defaults to: 0
	pub reminddays:Option<f64>,
	/// not nullable 
	pub supervisor_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub supervisor_id_ad_user:Option<AdUser>,
	/// has many
	pub ad_workflow:Option<Vec<AdWorkflow>>,
	/// has many
	pub ad_workflowprocessorlog:Option<Vec<AdWorkflowprocessorlog>>,
	/// has many
	pub pp_order_workflow:Option<Vec<PpOrderWorkflow>>,
}

pub struct AdWorkflowprocessorlog {
	/// primary
	/// not nullable 
	pub ad_workflowprocessor_id:f64,
	/// primary
	/// not nullable 
	pub ad_workflowprocessorlog_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub binarydata:Option<Vec<u8>>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub iserror:String,
	pub reference:Option<String>,
	pub summary:Option<String>,
	pub textmsg:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_workflowprocessor_id_ad_workflowprocessor:Option<AdWorkflowprocessor>,
}

pub struct AspClientexception {
	/// primary
	/// not nullable 
	pub asp_clientexception_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	pub ad_field_id:Option<f64>,
	pub ad_form_id:Option<f64>,
	/// defaults to: 0
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_process_id:Option<f64>,
	pub ad_process_para_id:Option<f64>,
	pub ad_tab_id:Option<f64>,
	pub ad_task_id:Option<f64>,
	pub ad_wf_node_id:Option<f64>,
	pub ad_window_id:Option<f64>,
	pub ad_workflow_id:Option<f64>,
	/// defaults to: 'U'::bpchar
	/// not nullable 
	pub asp_status:String,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_field_id_ad_field:Option<AdField>,
	/// has one
	pub ad_form_id_ad_form:Option<AdForm>,
	/// has one
	pub ad_process_id_ad_process:Option<AdProcess>,
	/// has one
	pub ad_process_para_id_ad_process_para:Option<AdProcessPara>,
	/// has one
	pub ad_tab_id_ad_tab:Option<AdTab>,
	/// has one
	pub ad_task_id_ad_task:Option<AdTask>,
	/// has one
	pub ad_wf_node_id_ad_wf_node:Option<AdWfNode>,
	/// has one
	pub ad_window_id_ad_window:Option<AdWindow>,
	/// has one
	pub ad_workflow_id_ad_workflow:Option<AdWorkflow>,
}

pub struct AspClientlevel {
	/// primary
	/// not nullable 
	pub asp_clientlevel_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// defaults to: 0
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub asp_level_id:f64,
	/// not nullable 
	pub asp_module_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub asp_level_id_asp_level:Option<AspLevel>,
	/// has one
	pub asp_module_id_asp_module:Option<AspModule>,
}

pub struct AspField {
	/// primary
	/// not nullable 
	pub asp_field_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_field_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: 'U'::bpchar
	/// not nullable 
	pub asp_status:String,
	pub asp_tab_id:Option<f64>,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_field_id_ad_field:Option<AdField>,
	/// has one
	pub asp_tab_id_asp_tab:Option<AspTab>,
}

pub struct AspForm {
	/// primary
	/// not nullable 
	pub asp_form_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_form_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub asp_level_id:f64,
	/// defaults to: 'U'::bpchar
	/// not nullable 
	pub asp_status:String,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_form_id_ad_form:Option<AdForm>,
	/// has one
	pub asp_level_id_asp_level:Option<AspLevel>,
}

pub struct AspLevel {
	/// primary
	/// not nullable 
	pub asp_level_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub asp_module_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	pub processing:Option<String>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
	/// has one
	pub asp_module_id_asp_module:Option<AspModule>,
	/// has many
	pub asp_clientlevel:Option<Vec<AspClientlevel>>,
	/// has many
	pub asp_form:Option<Vec<AspForm>>,
	/// has many
	pub asp_process:Option<Vec<AspProcess>>,
	/// has many
	pub asp_task:Option<Vec<AspTask>>,
	/// has many
	pub asp_window:Option<Vec<AspWindow>>,
	/// has many
	pub asp_workflow:Option<Vec<AspWorkflow>>,
}

pub struct AspModule {
	/// primary
	/// not nullable 
	pub asp_module_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
	/// has many
	pub asp_clientlevel:Option<Vec<AspClientlevel>>,
	/// has many
	pub asp_level:Option<Vec<AspLevel>>,
}

pub struct AspProcess {
	/// primary
	/// not nullable 
	pub asp_process_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_process_id:f64,
	/// not nullable 
	pub asp_level_id:f64,
	/// defaults to: 'U'::bpchar
	/// not nullable 
	pub asp_status:String,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_process_id_ad_process:Option<AdProcess>,
	/// has one
	pub asp_level_id_asp_level:Option<AspLevel>,
	/// has many
	pub asp_process_para:Option<Vec<AspProcessPara>>,
}

pub struct AspProcessPara {
	/// primary
	/// not nullable 
	pub asp_process_para_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_process_para_id:f64,
	pub asp_process_id:Option<f64>,
	/// defaults to: 'U'::bpchar
	/// not nullable 
	pub asp_status:String,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_process_para_id_ad_process_para:Option<AdProcessPara>,
	/// has one
	pub asp_process_id_asp_process:Option<AspProcess>,
}

pub struct AspTab {
	/// primary
	/// not nullable 
	pub asp_tab_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_tab_id:f64,
	/// defaults to: 'Y'::bpchar
	pub allfields:Option<String>,
	/// defaults to: 'U'::bpchar
	/// not nullable 
	pub asp_status:String,
	pub asp_window_id:Option<f64>,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub processing:Option<String>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_tab_id_ad_tab:Option<AdTab>,
	/// has one
	pub asp_window_id_asp_window:Option<AspWindow>,
	/// has many
	pub asp_field:Option<Vec<AspField>>,
}

pub struct AspTask {
	/// primary
	/// not nullable 
	pub asp_task_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_task_id:f64,
	/// not nullable 
	pub asp_level_id:f64,
	/// defaults to: 'U'::bpchar
	/// not nullable 
	pub asp_status:String,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_task_id_ad_task:Option<AdTask>,
	/// has one
	pub asp_level_id_asp_level:Option<AspLevel>,
}

pub struct AspWindow {
	/// primary
	/// not nullable 
	pub asp_window_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_window_id:f64,
	/// not nullable 
	pub asp_level_id:f64,
	/// defaults to: 'U'::bpchar
	/// not nullable 
	pub asp_status:String,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_window_id_ad_window:Option<AdWindow>,
	/// has one
	pub asp_level_id_asp_level:Option<AspLevel>,
	/// has many
	pub asp_tab:Option<Vec<AspTab>>,
}

pub struct AspWorkflow {
	/// primary
	/// not nullable 
	pub asp_workflow_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_workflow_id:f64,
	/// not nullable 
	pub asp_level_id:f64,
	/// defaults to: 'U'::bpchar
	/// not nullable 
	pub asp_status:String,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_workflow_id_ad_workflow:Option<AdWorkflow>,
	/// has one
	pub asp_level_id_asp_level:Option<AspLevel>,
}

pub struct Attribute {
	/// primary
	/// not nullable 
	pub attribute_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: NULL::numeric
	pub datatype_id:Option<f64>,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: NULL::numeric
	pub maxlength:Option<f64>,
	/// defaults to: NULL::numeric
	pub minlength:Option<f64>,
	/// not nullable 
	pub name:String,
	/// defaults to: NULL::numeric
	pub parent_id:Option<f64>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
}

pub struct AttributeView {
	/// primary
	/// not nullable 
	pub attribute_view_id:f64,
	/// defaults to: NULL::numeric
	pub ad_class_id:Option<f64>,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: NULL::numeric
	pub attribute_id:Option<f64>,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// defaults to: NULL::numeric
	pub view_id:Option<f64>,
}

pub struct BBid {
	/// primary
	/// not nullable 
	pub b_bid_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_user_id:f64,
	/// not nullable 
	pub b_buyerfunds_id:f64,
	/// not nullable 
	pub b_topic_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub iswillingtocommit:String,
	/// not nullable 
	pub name:String,
	pub privatenote:Option<String>,
	pub textmsg:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub b_topic_id_b_topic:Option<BTopic>,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
	/// has one
	pub ad_user_id_b_buyer:Option<BBuyer>,
	/// has one
	pub b_buyerfunds_id_b_buyerfunds:Option<BBuyerfunds>,
}

pub struct BBidcomment {
	/// primary
	/// not nullable 
	pub b_topic_id:f64,
	/// primary
	/// not nullable 
	pub b_bidcomment_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_user_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub textmsg:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub b_topic_id_b_topic:Option<BTopic>,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
}

pub struct BBuyer {
	/// primary
	/// not nullable 
	pub ad_user_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub validto:NaiveDateTime,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
	/// has many
	pub b_bid:Option<Vec<BBid>>,
	/// has many
	pub b_buyerfunds:Option<Vec<BBuyerfunds>>,
}

pub struct BBuyerfunds {
	/// primary
	/// not nullable 
	pub b_buyerfunds_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_user_id:f64,
	pub c_order_id:Option<f64>,
	pub c_payment_id:Option<f64>,
	/// defaults to: 0
	/// not nullable 
	pub committedamt:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 0
	/// not nullable 
	pub noncommittedamt:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
	/// has one
	pub ad_user_id_b_buyer:Option<BBuyer>,
	/// has one
	pub c_order_id_c_order:Option<COrder>,
	/// has one
	pub c_payment_id_c_payment:Option<CPayment>,
	/// has many
	pub b_bid:Option<Vec<BBid>>,
}

pub struct BOffer {
	/// primary
	/// not nullable 
	pub b_offer_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_user_id:f64,
	/// not nullable 
	pub b_sellerfunds_id:f64,
	/// not nullable 
	pub b_topic_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub iswillingtocommit:String,
	/// not nullable 
	pub name:String,
	pub privatenote:Option<String>,
	pub textmsg:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub b_topic_id_b_topic:Option<BTopic>,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
	/// has one
	pub ad_user_id_b_seller:Option<BSeller>,
	/// has one
	pub b_sellerfunds_id_b_sellerfunds:Option<BSellerfunds>,
}

pub struct BSeller {
	/// primary
	/// not nullable 
	pub ad_user_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isinternal:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub validto:NaiveDateTime,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
	/// has many
	pub b_offer:Option<Vec<BOffer>>,
	/// has many
	pub b_sellerfunds:Option<Vec<BSellerfunds>>,
}

pub struct BSellerfunds {
	/// primary
	/// not nullable 
	pub b_sellerfunds_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_user_id:f64,
	pub c_order_id:Option<f64>,
	pub c_payment_id:Option<f64>,
	/// defaults to: 0
	/// not nullable 
	pub committedamt:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 0
	/// not nullable 
	pub noncommittedamt:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
	/// has one
	pub ad_user_id_b_seller:Option<BSeller>,
	/// has one
	pub c_order_id_c_order:Option<COrder>,
	/// has one
	pub c_payment_id_c_payment:Option<CPayment>,
	/// has many
	pub b_offer:Option<Vec<BOffer>>,
}

pub struct BTopic {
	/// primary
	/// not nullable 
	pub b_topic_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub b_topiccategory_id:f64,
	/// not nullable 
	pub b_topictype_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub decisiondate:NaiveDateTime,
	pub description:Option<String>,
	/// not nullable 
	pub documentno:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub ispublished:String,
	/// not nullable 
	pub name:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	pub processing:Option<String>,
	pub textdetails:Option<String>,
	pub textmsg:Option<String>,
	/// not nullable 
	pub topicaction:String,
	/// not nullable 
	pub topicstatus:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub b_topictype_id_b_topictype:Option<BTopictype>,
	/// has one
	pub b_topiccategory_id_b_topiccategory:Option<BTopiccategory>,
	/// has many
	pub b_bid:Option<Vec<BBid>>,
	/// has many
	pub b_bidcomment:Option<Vec<BBidcomment>>,
	/// has many
	pub b_offer:Option<Vec<BOffer>>,
}

pub struct BTopiccategory {
	/// primary
	/// not nullable 
	pub b_topiccategory_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub b_topictype_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub b_topictype_id_b_topictype:Option<BTopictype>,
	/// has many
	pub b_topic:Option<Vec<BTopic>>,
}

pub struct BTopictype {
	/// primary
	/// not nullable 
	pub b_topictype_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub auctiontype:String,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub m_pricelist_id:f64,
	/// not nullable 
	pub m_product_id:f64,
	/// not nullable 
	pub m_productmember_id:f64,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub m_pricelist_id_m_pricelist:Option<MPricelist>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub m_productmember_id_m_product:Option<MProduct>,
	/// has many
	pub b_topic:Option<Vec<BTopic>>,
	/// has many
	pub b_topiccategory:Option<Vec<BTopiccategory>>,
}

pub struct CAcctprocessor {
	/// primary
	/// not nullable 
	pub c_acctprocessor_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_table_id:Option<f64>,
	pub c_acctschema_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub datelastrun:Option<NaiveDateTime>,
	pub datenextrun:Option<NaiveDateTime>,
	pub description:Option<String>,
	/// not nullable 
	pub frequency:f64,
	/// not nullable 
	pub frequencytype:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub keeplogdays:f64,
	/// not nullable 
	pub name:String,
	pub processing:Option<String>,
	/// not nullable 
	pub supervisor_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub supervisor_id_ad_user:Option<AdUser>,
	/// has one
	pub c_acctschema_id_c_acctschema:Option<CAcctschema>,
	/// has one
	pub ad_table_id_ad_table:Option<AdTable>,
	/// has many
	pub c_acctprocessorlog:Option<Vec<CAcctprocessorlog>>,
}

pub struct CAcctprocessorlog {
	/// primary
	/// not nullable 
	pub c_acctprocessor_id:f64,
	/// primary
	/// not nullable 
	pub c_acctprocessorlog_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub binarydata:Option<Vec<u8>>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub iserror:String,
	pub reference:Option<String>,
	pub summary:Option<String>,
	pub textmsg:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_acctprocessor_id_c_acctprocessor:Option<CAcctprocessor>,
}

pub struct CAcctschema {
	/// primary
	/// not nullable 
	pub c_acctschema_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_orgonly_id:Option<f64>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub autoperiodcontrol:String,
	/// not nullable 
	pub c_currency_id:f64,
	pub c_period_id:Option<f64>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub commitmenttype:String,
	/// defaults to: 'C'::bpchar
	/// not nullable 
	pub costinglevel:String,
	/// not nullable 
	pub costingmethod:String,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// not nullable 
	pub gaap:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub hasalias:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub hascombination:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isaccrual:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isadjustcogs:String,
	/// defaults to: 'Y'::bpchar
	pub isallownegativeposting:Option<String>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isdiscountcorrectstax:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isexplicitcostadjustment:String,
	/// defaults to: 'Y'::bpchar
	pub ispostifclearingequal:Option<String>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub ispostservices:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub istradediscountposted:String,
	pub m_costtype_id:Option<f64>,
	/// not nullable 
	pub name:String,
	pub period_openfuture:Option<f64>,
	pub period_openhistory:Option<f64>,
	pub processing:Option<String>,
	/// not nullable 
	pub separator:String,
	pub taxcorrectiontype:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_client_id_ad_client:Option<AdClient>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub c_currency_id_c_currency:Option<CCurrency>,
	/// has one
	pub c_period_id_c_period:Option<CPeriod>,
	/// has one
	pub m_costtype_id_m_costtype:Option<MCosttype>,
	/// has one
	pub ad_orgonly_id_ad_org:Option<AdOrg>,
	/// has many
	pub a_asset_acct:Option<Vec<AAssetAcct>>,
	/// has many
	pub a_asset_change:Option<Vec<AAssetChange>>,
	/// has many
	pub a_asset_change_amt:Option<Vec<AAssetChangeAmt>>,
	/// has many
	pub a_asset_group_acct:Option<Vec<AAssetGroupAcct>>,
	/// has many
	pub a_asset_reval_entry:Option<Vec<AAssetRevalEntry>>,
	/// has many
	pub a_asset_transfer:Option<Vec<AAssetTransfer>>,
	/// has many
	pub a_depreciation_entry:Option<Vec<ADepreciationEntry>>,
	/// has many
	pub ad_clientinfo:Option<Vec<AdClientinfo>>,
	/// has many
	pub c_acctprocessor:Option<Vec<CAcctprocessor>>,
	/// has many
	pub c_acctschema_default:Option<Vec<CAcctschemaDefault>>,
	/// has many
	pub c_acctschema_element:Option<Vec<CAcctschemaElement>>,
	/// has many
	pub c_acctschema_gl:Option<Vec<CAcctschemaGl>>,
	/// has many
	pub c_bankaccount_acct:Option<Vec<CBankaccountAcct>>,
	/// has many
	pub c_bp_customer_acct:Option<Vec<CBpCustomerAcct>>,
	/// has many
	pub c_bp_employee_acct:Option<Vec<CBpEmployeeAcct>>,
	/// has many
	pub c_bp_group_acct:Option<Vec<CBpGroupAcct>>,
	/// has many
	pub c_bp_vendor_acct:Option<Vec<CBpVendorAcct>>,
	/// has many
	pub c_cashbook_acct:Option<Vec<CCashbookAcct>>,
	/// has many
	pub c_charge_acct:Option<Vec<CChargeAcct>>,
	/// has many
	pub c_currency_acct:Option<Vec<CCurrencyAcct>>,
	/// has many
	pub c_interorg_acct:Option<Vec<CInterorgAcct>>,
	/// has many
	pub c_project_acct:Option<Vec<CProjectAcct>>,
	/// has many
	pub c_revenuerecognition_plan:Option<Vec<CRevenuerecognitionPlan>>,
	/// has many
	pub c_tax_acct:Option<Vec<CTaxAcct>>,
	/// has many
	pub c_taxdeclarationacct:Option<Vec<CTaxdeclarationacct>>,
	/// has many
	pub c_validcombination:Option<Vec<CValidcombination>>,
	/// has many
	pub c_withholding_acct:Option<Vec<CWithholdingAcct>>,
	/// has many
	pub fact_acct:Option<Vec<FactAcct>>,
	/// has many
	pub gl_budgetcontrol:Option<Vec<GlBudgetcontrol>>,
	/// has many
	pub gl_distribution:Option<Vec<GlDistribution>>,
	/// has many
	pub gl_fund:Option<Vec<GlFund>>,
	/// has many
	pub gl_journal:Option<Vec<GlJournal>>,
	/// has many
	pub hr_concept_acct:Option<Vec<HrConceptAcct>>,
	/// has many
	pub i_asset:Option<Vec<IAsset>>,
	/// has many
	pub i_fajournal:Option<Vec<IFajournal>>,
	/// has many
	pub i_gljournal:Option<Vec<IGljournal>>,
	/// has many
	pub m_cost:Option<Vec<MCost>>,
	/// has many
	pub m_costdetail:Option<Vec<MCostdetail>>,
	/// has many
	pub m_costqueue:Option<Vec<MCostqueue>>,
	/// has many
	pub m_product_acct:Option<Vec<MProductAcct>>,
	/// has many
	pub m_product_category_acct:Option<Vec<MProductCategoryAcct>>,
	/// has many
	pub m_product_costing:Option<Vec<MProductCosting>>,
	/// has many
	pub m_warehouse_acct:Option<Vec<MWarehouseAcct>>,
	/// has many
	pub pa_ratio:Option<Vec<PaRatio>>,
	/// has many
	pub pa_report:Option<Vec<PaReport>>,
	/// has many
	pub pp_order_cost:Option<Vec<PpOrderCost>>,
}

pub struct CAcctschemaDefault {
	/// primary
	/// not nullable 
	pub c_acctschema_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub b_asset_acct:f64,
	/// not nullable 
	pub b_expense_acct:f64,
	/// not nullable 
	pub b_interestexp_acct:f64,
	/// not nullable 
	pub b_interestrev_acct:f64,
	/// not nullable 
	pub b_intransit_acct:f64,
	/// not nullable 
	pub b_paymentselect_acct:f64,
	/// not nullable 
	pub b_revaluationgain_acct:f64,
	/// not nullable 
	pub b_revaluationloss_acct:f64,
	/// not nullable 
	pub b_settlementgain_acct:f64,
	/// not nullable 
	pub b_settlementloss_acct:f64,
	/// not nullable 
	pub b_unallocatedcash_acct:f64,
	/// not nullable 
	pub b_unidentified_acct:f64,
	/// not nullable 
	pub c_prepayment_acct:f64,
	/// not nullable 
	pub c_receivable_acct:f64,
	/// not nullable 
	pub c_receivable_services_acct:f64,
	/// not nullable 
	pub cb_asset_acct:f64,
	/// not nullable 
	pub cb_cashtransfer_acct:f64,
	/// not nullable 
	pub cb_differences_acct:f64,
	/// not nullable 
	pub cb_expense_acct:f64,
	/// not nullable 
	pub cb_receipt_acct:f64,
	/// not nullable 
	pub ch_expense_acct:f64,
	/// not nullable 
	pub ch_revenue_acct:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub e_expense_acct:f64,
	/// not nullable 
	pub e_prepayment_acct:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub notinvoicedreceipts_acct:f64,
	/// not nullable 
	pub notinvoicedreceivables_acct:f64,
	/// not nullable 
	pub notinvoicedrevenue_acct:f64,
	/// not nullable 
	pub p_asset_acct:f64,
	pub p_burden_acct:Option<f64>,
	/// not nullable 
	pub p_cogs_acct:f64,
	/// not nullable 
	pub p_costadjustment_acct:f64,
	pub p_costofproduction_acct:Option<f64>,
	/// not nullable 
	pub p_expense_acct:f64,
	pub p_floorstock_acct:Option<f64>,
	/// not nullable 
	pub p_inventoryclearing_acct:f64,
	/// not nullable 
	pub p_invoicepricevariance_acct:f64,
	pub p_labor_acct:Option<f64>,
	pub p_methodchangevariance_acct:Option<f64>,
	pub p_mixvariance_acct:Option<f64>,
	pub p_outsideprocessing_acct:Option<f64>,
	pub p_overhead_acct:Option<f64>,
	/// not nullable 
	pub p_purchasepricevariance_acct:f64,
	pub p_ratevariance_acct:Option<f64>,
	/// not nullable 
	pub p_revenue_acct:f64,
	pub p_scrap_acct:Option<f64>,
	/// not nullable 
	pub p_tradediscountgrant_acct:f64,
	/// not nullable 
	pub p_tradediscountrec_acct:f64,
	pub p_usagevariance_acct:Option<f64>,
	pub p_wip_acct:Option<f64>,
	/// not nullable 
	pub paydiscount_exp_acct:f64,
	/// not nullable 
	pub paydiscount_rev_acct:f64,
	/// not nullable 
	pub pj_asset_acct:f64,
	/// not nullable 
	pub pj_wip_acct:f64,
	pub processing:Option<String>,
	/// not nullable 
	pub realizedgain_acct:f64,
	/// not nullable 
	pub realizedloss_acct:f64,
	/// not nullable 
	pub t_credit_acct:f64,
	/// not nullable 
	pub t_due_acct:f64,
	/// not nullable 
	pub t_expense_acct:f64,
	/// not nullable 
	pub t_liability_acct:f64,
	/// not nullable 
	pub t_receivables_acct:f64,
	/// not nullable 
	pub unearnedrevenue_acct:f64,
	/// not nullable 
	pub unrealizedgain_acct:f64,
	/// not nullable 
	pub unrealizedloss_acct:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub v_liability_acct:f64,
	/// not nullable 
	pub v_liability_services_acct:f64,
	/// not nullable 
	pub v_prepayment_acct:f64,
	/// not nullable 
	pub w_differences_acct:f64,
	/// not nullable 
	pub w_invactualadjust_acct:f64,
	/// not nullable 
	pub w_inventory_acct:f64,
	/// not nullable 
	pub w_revaluation_acct:f64,
	/// not nullable 
	pub withholding_acct:f64,
	/// not nullable 
	pub writeoff_acct:f64,
	/// has one
	pub c_acctschema_id_c_acctschema:Option<CAcctschema>,
	/// has one
	pub w_inventory_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub w_invactualadjust_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub w_differences_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub w_revaluation_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub p_revenue_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub p_expense_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub p_asset_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub p_purchasepricevariance_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub p_invoicepricevariance_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub p_tradediscountrec_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub p_tradediscountgrant_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub p_cogs_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub c_receivable_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub c_prepayment_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub v_liability_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub v_liability_services_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub v_prepayment_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub paydiscount_exp_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub writeoff_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub paydiscount_rev_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub unrealizedgain_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub unrealizedloss_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub realizedgain_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub realizedloss_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub withholding_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub e_prepayment_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub e_expense_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub pj_asset_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub pj_wip_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub t_expense_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub t_liability_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub t_receivables_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub t_due_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub t_credit_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub b_intransit_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub b_asset_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub b_expense_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub b_interestrev_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub b_interestexp_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub b_unidentified_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub b_unallocatedcash_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub b_paymentselect_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub b_settlementgain_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub b_settlementloss_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub b_revaluationgain_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub b_revaluationloss_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub ch_expense_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub ch_revenue_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub unearnedrevenue_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub notinvoicedreceivables_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub notinvoicedrevenue_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub notinvoicedreceipts_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub cb_asset_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub cb_cashtransfer_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub cb_differences_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub cb_expense_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub cb_receipt_acct_c_validcombination:Option<CValidcombination>,
}

pub struct CAcctschemaElement {
	/// primary
	/// not nullable 
	pub c_acctschema_element_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	pub ad_column_id:Option<f64>,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_acctschema_id:f64,
	pub c_activity_id:Option<f64>,
	pub c_bpartner_id:Option<f64>,
	pub c_campaign_id:Option<f64>,
	pub c_element_id:Option<f64>,
	pub c_elementvalue_id:Option<f64>,
	pub c_location_id:Option<f64>,
	pub c_project_id:Option<f64>,
	pub c_salesregion_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub elementtype:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isbalanced:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub ismandatory:String,
	pub m_product_id:Option<f64>,
	/// not nullable 
	pub name:String,
	pub org_id:Option<f64>,
	/// not nullable 
	pub seqno:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub c_acctschema_id_c_acctschema:Option<CAcctschema>,
	/// has one
	pub c_element_id_c_element:Option<CElement>,
	/// has one
	pub ad_client_id_ad_client:Option<AdClient>,
	/// has one
	pub org_id_ad_org:Option<AdOrg>,
	/// has one
	pub c_elementvalue_id_c_elementvalue:Option<CElementvalue>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub c_location_id_c_location:Option<CLocation>,
	/// has one
	pub c_salesregion_id_c_salesregion:Option<CSalesregion>,
	/// has one
	pub c_project_id_c_project:Option<CProject>,
	/// has one
	pub c_campaign_id_c_campaign:Option<CCampaign>,
	/// has one
	pub c_activity_id_c_activity:Option<CActivity>,
	/// has one
	pub ad_column_id_ad_column:Option<AdColumn>,
}

pub struct CAcctschemaGl {
	/// primary
	/// not nullable 
	pub c_acctschema_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub commitmentoffset_acct:f64,
	/// not nullable 
	pub commitmentoffsetsales_acct:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub currencybalancing_acct:Option<f64>,
	/// not nullable 
	pub incomesummary_acct:f64,
	/// not nullable 
	pub intercompanyduefrom_acct:f64,
	/// not nullable 
	pub intercompanydueto_acct:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub ppvoffset_acct:f64,
	/// not nullable 
	pub retainedearning_acct:f64,
	pub suspensebalancing_acct:Option<f64>,
	pub suspenseerror_acct:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub usecurrencybalancing:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub usesuspensebalancing:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub usesuspenseerror:String,
	/// has one
	pub c_acctschema_id_c_acctschema:Option<CAcctschema>,
	/// has one
	pub suspensebalancing_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub suspenseerror_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub currencybalancing_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub retainedearning_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub incomesummary_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub intercompanydueto_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub intercompanyduefrom_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub ppvoffset_acct_c_validcombination:Option<CValidcombination>,
}

pub struct CActivity {
	/// primary
	/// not nullable 
	pub c_activity_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub issummary:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
	/// has many
	pub c_acctschema_element:Option<Vec<CAcctschemaElement>>,
	/// has many
	pub c_cash:Option<Vec<CCash>>,
	/// has many
	pub c_invoice:Option<Vec<CInvoice>>,
	/// has many
	pub c_invoicebatchline:Option<Vec<CInvoicebatchline>>,
	/// has many
	pub c_invoiceline:Option<Vec<CInvoiceline>>,
	/// has many
	pub c_order:Option<Vec<COrder>>,
	/// has many
	pub c_orderline:Option<Vec<COrderline>>,
	/// has many
	pub c_payment:Option<Vec<CPayment>>,
	/// has many
	pub c_validcombination:Option<Vec<CValidcombination>>,
	/// has many
	pub dd_order:Option<Vec<DdOrder>>,
	/// has many
	pub dd_orderline:Option<Vec<DdOrderline>>,
	/// has many
	pub fact_acct:Option<Vec<FactAcct>>,
	/// has many
	pub gl_distribution:Option<Vec<GlDistribution>>,
	/// has many
	pub gl_distributionline:Option<Vec<GlDistributionline>>,
	/// has many
	pub hr_department:Option<Vec<HrDepartment>>,
	/// has many
	pub hr_employee:Option<Vec<HrEmployee>>,
	/// has many
	pub hr_movement:Option<Vec<HrMovement>>,
	/// has many
	pub i_fajournal:Option<Vec<IFajournal>>,
	/// has many
	pub i_gljournal:Option<Vec<IGljournal>>,
	/// has many
	pub i_invoice:Option<Vec<IInvoice>>,
	/// has many
	pub i_order:Option<Vec<IOrder>>,
	/// has many
	pub m_inout:Option<Vec<MInout>>,
	/// has many
	pub m_inoutline:Option<Vec<MInoutline>>,
	/// has many
	pub m_inventory:Option<Vec<MInventory>>,
	/// has many
	pub m_movement:Option<Vec<MMovement>>,
	/// has many
	pub m_production:Option<Vec<MProduction>>,
	/// has many
	pub pa_reportcolumn:Option<Vec<PaReportcolumn>>,
	/// has many
	pub pa_reportsource:Option<Vec<PaReportsource>>,
	/// has many
	pub pp_cost_collector:Option<Vec<PpCostCollector>>,
	/// has many
	pub pp_order:Option<Vec<PpOrder>>,
	/// has many
	pub r_request:Option<Vec<RRequest>>,
	/// has many
	pub r_requestaction:Option<Vec<RRequestaction>>,
	/// has many
	pub s_timeexpenseline:Option<Vec<STimeexpenseline>>,
	/// has many
	pub t_aging:Option<Vec<TAging>>,
}

pub struct CAllocationhdr {
	/// primary
	/// not nullable 
	pub c_allocationhdr_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: 0
	/// not nullable 
	pub approvalamt:f64,
	/// not nullable 
	pub c_currency_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub dateacct:NaiveDateTime,
	/// not nullable 
	pub datetrx:NaiveDateTime,
	pub description:Option<String>,
	/// not nullable 
	pub docaction:String,
	/// not nullable 
	pub docstatus:String,
	/// not nullable 
	pub documentno:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isapproved:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub ismanual:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub posted:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	pub processing:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_currency_id_c_currency:Option<CCurrency>,
	/// has many
	pub c_allocationline:Option<Vec<CAllocationline>>,
}

pub struct CAllocationline {
	/// primary
	/// not nullable 
	pub c_allocationline_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub allocationno:Option<f64>,
	/// defaults to: 0
	/// not nullable 
	pub amount:f64,
	/// not nullable 
	pub c_allocationhdr_id:f64,
	pub c_bpartner_id:Option<f64>,
	pub c_cashline_id:Option<f64>,
	pub c_invoice_id:Option<f64>,
	pub c_order_id:Option<f64>,
	pub c_payment_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub datetrx:Option<NaiveDateTime>,
	/// defaults to: 0
	/// not nullable 
	pub discountamt:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	pub ismanual:Option<String>,
	/// defaults to: 0
	pub overunderamt:Option<f64>,
	/// defaults to: 'N'::bpchar
	pub posted:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// defaults to: 0
	/// not nullable 
	pub writeoffamt:f64,
	/// has one
	pub c_invoice_id_c_invoice:Option<CInvoice>,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub c_order_id_c_order:Option<COrder>,
	/// has one
	pub c_payment_id_c_payment:Option<CPayment>,
	/// has one
	pub c_cashline_id_c_cashline:Option<CCashline>,
	/// has one
	pub c_allocationhdr_id_c_allocationhdr:Option<CAllocationhdr>,
	/// has many
	pub c_paymentallocate:Option<Vec<CPaymentallocate>>,
	/// has many
	pub c_taxdeclarationline:Option<Vec<CTaxdeclarationline>>,
}

pub struct CBank {
	/// primary
	/// not nullable 
	pub c_bank_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub c_location_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isownbank:String,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub routingno:String,
	pub swiftcode:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_location_id_c_location:Option<CLocation>,
	/// has many
	pub ad_orginfo:Option<Vec<AdOrginfo>>,
	/// has many
	pub c_bankaccount:Option<Vec<CBankaccount>>,
	/// has many
	pub c_bp_bankaccount:Option<Vec<CBpBankaccount>>,
}

pub struct CBankaccount {
	/// primary
	/// not nullable 
	pub c_bankaccount_id:f64,
	/// not nullable 
	pub accountno:String,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub bankaccounttype:String,
	pub bban:Option<String>,
	/// not nullable 
	pub c_bank_id:f64,
	/// not nullable 
	pub c_currency_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 0
	/// not nullable 
	pub creditlimit:f64,
	/// defaults to: 0
	/// not nullable 
	pub currentbalance:f64,
	pub description:Option<String>,
	pub iban:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isdefault:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_bank_id_c_bank:Option<CBank>,
	/// has one
	pub c_currency_id_c_currency:Option<CCurrency>,
	/// has many
	pub c_bankaccount_acct:Option<Vec<CBankaccountAcct>>,
	/// has many
	pub c_bankaccountdoc:Option<Vec<CBankaccountdoc>>,
	/// has many
	pub c_bankstatement:Option<Vec<CBankstatement>>,
	/// has many
	pub c_bankstatementloader:Option<Vec<CBankstatementloader>>,
	/// has many
	pub c_cashline:Option<Vec<CCashline>>,
	/// has many
	pub c_elementvalue:Option<Vec<CElementvalue>>,
	/// has many
	pub c_payment:Option<Vec<CPayment>>,
	/// has many
	pub c_paymentprocessor:Option<Vec<CPaymentprocessor>>,
	/// has many
	pub c_payselection:Option<Vec<CPayselection>>,
	/// has many
	pub c_pos:Option<Vec<CPos>>,
	/// has many
	pub i_bankstatement:Option<Vec<IBankstatement>>,
	/// has many
	pub i_payment:Option<Vec<IPayment>>,
}

pub struct CBankaccountAcct {
	/// primary
	/// not nullable 
	pub c_bankaccount_id:f64,
	/// primary
	/// not nullable 
	pub c_acctschema_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub b_asset_acct:f64,
	/// not nullable 
	pub b_expense_acct:f64,
	/// not nullable 
	pub b_interestexp_acct:f64,
	/// not nullable 
	pub b_interestrev_acct:f64,
	/// not nullable 
	pub b_intransit_acct:f64,
	/// not nullable 
	pub b_paymentselect_acct:f64,
	/// not nullable 
	pub b_revaluationgain_acct:f64,
	/// not nullable 
	pub b_revaluationloss_acct:f64,
	/// not nullable 
	pub b_settlementgain_acct:f64,
	/// not nullable 
	pub b_settlementloss_acct:f64,
	/// not nullable 
	pub b_unallocatedcash_acct:f64,
	/// not nullable 
	pub b_unidentified_acct:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_bankaccount_id_c_bankaccount:Option<CBankaccount>,
	/// has one
	pub c_acctschema_id_c_acctschema:Option<CAcctschema>,
	/// has one
	pub b_intransit_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub b_asset_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub b_expense_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub b_interestrev_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub b_interestexp_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub b_unidentified_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub b_unallocatedcash_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub b_paymentselect_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub b_settlementgain_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub b_settlementloss_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub b_revaluationgain_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub b_revaluationloss_acct_c_validcombination:Option<CValidcombination>,
}

pub struct CBankaccountdoc {
	/// primary
	/// not nullable 
	pub c_bankaccountdoc_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_bankaccount_id:f64,
	pub check_printformat_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub currentnext:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub paymentrule:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_bankaccount_id_c_bankaccount:Option<CBankaccount>,
	/// has one
	pub check_printformat_id_ad_printformat:Option<AdPrintformat>,
}

pub struct CBankstatement {
	/// primary
	/// not nullable 
	pub c_bankstatement_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: 0
	pub beginningbalance:Option<f64>,
	/// not nullable 
	pub c_bankaccount_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'N'::bpchar
	pub createfrom:Option<String>,
	pub description:Option<String>,
	/// not nullable 
	pub docaction:String,
	/// not nullable 
	pub docstatus:String,
	pub eftstatementdate:Option<NaiveDateTime>,
	pub eftstatementreference:Option<String>,
	/// defaults to: 0
	/// not nullable 
	pub endingbalance:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isapproved:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub ismanual:String,
	pub matchstatement:Option<String>,
	/// not nullable 
	pub name:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub posted:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	pub processing:Option<String>,
	/// not nullable 
	pub statementdate:NaiveDateTime,
	/// defaults to: 0
	pub statementdifference:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_bankaccount_id_c_bankaccount:Option<CBankaccount>,
	/// has many
	pub c_bankstatementline:Option<Vec<CBankstatementline>>,
	/// has many
	pub i_bankstatement:Option<Vec<IBankstatement>>,
}

pub struct CBankstatementline {
	/// primary
	/// not nullable 
	pub c_bankstatementline_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_bankstatement_id:f64,
	pub c_bpartner_id:Option<f64>,
	pub c_charge_id:Option<f64>,
	/// not nullable 
	pub c_currency_id:f64,
	pub c_invoice_id:Option<f64>,
	pub c_payment_id:Option<f64>,
	/// defaults to: 0
	/// not nullable 
	pub chargeamt:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub createpayment:Option<String>,
	/// not nullable 
	pub dateacct:NaiveDateTime,
	pub description:Option<String>,
	/// defaults to: 0
	pub eftamt:Option<f64>,
	pub eftcheckno:Option<String>,
	pub eftcurrency:Option<String>,
	pub eftmemo:Option<String>,
	pub eftpayee:Option<String>,
	pub eftpayeeaccount:Option<String>,
	pub eftreference:Option<String>,
	pub eftstatementlinedate:Option<NaiveDateTime>,
	pub efttrxid:Option<String>,
	pub efttrxtype:Option<String>,
	pub eftvalutadate:Option<NaiveDateTime>,
	/// defaults to: 0
	/// not nullable 
	pub interestamt:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub ismanual:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isreversal:String,
	/// not nullable 
	pub line:f64,
	pub matchstatement:Option<String>,
	pub memo:Option<String>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	pub referenceno:Option<String>,
	/// not nullable 
	pub statementlinedate:NaiveDateTime,
	/// defaults to: 0
	/// not nullable 
	pub stmtamt:f64,
	/// defaults to: 0
	/// not nullable 
	pub trxamt:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub valutadate:NaiveDateTime,
	/// has one
	pub c_bankstatement_id_c_bankstatement:Option<CBankstatement>,
	/// has one
	pub c_payment_id_c_payment:Option<CPayment>,
	/// has one
	pub c_currency_id_c_currency:Option<CCurrency>,
	/// has one
	pub c_charge_id_c_charge:Option<CCharge>,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub c_invoice_id_c_invoice:Option<CInvoice>,
	/// has many
	pub i_bankstatement:Option<Vec<IBankstatement>>,
}

pub struct CBankstatementloader {
	/// primary
	/// not nullable 
	pub c_bankstatementloader_id:f64,
	pub accountno:Option<String>,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub branchid:Option<String>,
	/// not nullable 
	pub c_bankaccount_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub dateformat:Option<String>,
	pub datelastrun:Option<NaiveDateTime>,
	pub description:Option<String>,
	pub filename:Option<String>,
	pub financialinstitutionid:Option<String>,
	pub hostaddress:Option<String>,
	pub hostport:Option<f64>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	pub password:Option<String>,
	pub pin:Option<String>,
	pub proxyaddress:Option<String>,
	pub proxylogon:Option<String>,
	pub proxypassword:Option<String>,
	pub proxyport:Option<f64>,
	pub stmtloaderclass:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub userid:Option<String>,
	/// has one
	pub c_bankaccount_id_c_bankaccount:Option<CBankaccount>,
}

pub struct CBankstatementmatcher {
	/// primary
	/// not nullable 
	pub c_bankstatementmatcher_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub classname:String,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub seqno:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
}

pub struct CBpBankaccount {
	/// primary
	/// not nullable 
	pub c_bp_bankaccount_id:f64,
	pub a_city:Option<String>,
	pub a_country:Option<String>,
	pub a_email:Option<String>,
	pub a_ident_dl:Option<String>,
	pub a_ident_ssn:Option<String>,
	pub a_name:Option<String>,
	pub a_state:Option<String>,
	pub a_street:Option<String>,
	pub a_zip:Option<String>,
	pub accountno:Option<String>,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_user_id:Option<f64>,
	pub bankaccounttype:Option<String>,
	pub bpbankacctuse:Option<String>,
	pub c_bank_id:Option<f64>,
	/// not nullable 
	pub c_bpartner_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub creditcardexpmm:Option<f64>,
	pub creditcardexpyy:Option<f64>,
	pub creditcardnumber:Option<String>,
	pub creditcardtype:Option<String>,
	pub creditcardvv:Option<String>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isach:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub r_avsaddr:Option<String>,
	pub r_avszip:Option<String>,
	pub routingno:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub c_bank_id_c_bank:Option<CBank>,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
	/// has many
	pub c_payment:Option<Vec<CPayment>>,
	/// has many
	pub c_payselectioncheck:Option<Vec<CPayselectioncheck>>,
}

pub struct CBpCustomerAcct {
	/// primary
	/// not nullable 
	pub c_bpartner_id:f64,
	/// primary
	/// not nullable 
	pub c_acctschema_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub c_prepayment_acct:Option<f64>,
	pub c_receivable_acct:Option<f64>,
	pub c_receivable_services_acct:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub c_acctschema_id_c_acctschema:Option<CAcctschema>,
	/// has one
	pub c_receivable_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub c_prepayment_acct_c_validcombination:Option<CValidcombination>,
}

pub struct CBpEdi {
	/// primary
	/// not nullable 
	pub c_bp_edi_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_sequence_id:f64,
	/// not nullable 
	pub c_bpartner_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub customerno:String,
	pub description:Option<String>,
	/// not nullable 
	pub editype:String,
	/// not nullable 
	pub email_error_to:String,
	pub email_from:Option<String>,
	pub email_from_pwd:Option<String>,
	pub email_from_uid:Option<String>,
	/// not nullable 
	pub email_info_to:String,
	pub email_to:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isaudited:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isinfosent:String,
	/// not nullable 
	pub m_warehouse_id:f64,
	/// not nullable 
	pub name:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub receiveinquiryreply:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub receiveorderreply:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub sendinquiry:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub sendorder:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub m_warehouse_id_m_warehouse:Option<MWarehouse>,
	/// has one
	pub ad_sequence_id_ad_sequence:Option<AdSequence>,
	/// has many
	pub m_edi:Option<Vec<MEdi>>,
}

pub struct CBpEmployeeAcct {
	/// primary
	/// not nullable 
	pub c_bpartner_id:f64,
	/// primary
	/// not nullable 
	pub c_acctschema_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub e_expense_acct:Option<f64>,
	pub e_prepayment_acct:Option<f64>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub c_acctschema_id_c_acctschema:Option<CAcctschema>,
	/// has one
	pub e_expense_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub e_prepayment_acct_c_validcombination:Option<CValidcombination>,
}

pub struct CBpGroup {
	/// primary
	/// not nullable 
	pub c_bp_group_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_printcolor_id:Option<f64>,
	pub c_dunning_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub creditwatchpercent:Option<f64>,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isconfidentialinfo:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isdefault:String,
	pub m_discountschema_id:Option<f64>,
	pub m_pricelist_id:Option<f64>,
	/// not nullable 
	pub name:String,
	pub po_discountschema_id:Option<f64>,
	pub po_pricelist_id:Option<f64>,
	pub pricematchtolerance:Option<f64>,
	pub prioritybase:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
	/// has one
	pub ad_printcolor_id_ad_printcolor:Option<AdPrintcolor>,
	/// has one
	pub m_pricelist_id_m_pricelist:Option<MPricelist>,
	/// has one
	pub po_pricelist_id_m_pricelist:Option<MPricelist>,
	/// has one
	pub m_discountschema_id_m_discountschema:Option<MDiscountschema>,
	/// has one
	pub po_discountschema_id_m_discountschema:Option<MDiscountschema>,
	/// has one
	pub c_dunning_id_c_dunning:Option<CDunning>,
	/// has many
	pub c_bp_group_acct:Option<Vec<CBpGroupAcct>>,
	/// has many
	pub c_bpartner:Option<Vec<CBpartner>>,
	/// has many
	pub c_commissionline:Option<Vec<CCommissionline>>,
	/// has many
	pub c_taxdefinition:Option<Vec<CTaxdefinition>>,
	/// has many
	pub cm_accesslistbpgroup:Option<Vec<CmAccesslistbpgroup>>,
	/// has many
	pub hr_concept_acct:Option<Vec<HrConceptAcct>>,
	/// has many
	pub i_bpartner:Option<Vec<IBpartner>>,
	/// has many
	pub pa_goalrestriction:Option<Vec<PaGoalrestriction>>,
	/// has many
	pub t_aging:Option<Vec<TAging>>,
}

pub struct CBpGroupAcct {
	/// primary
	/// not nullable 
	pub c_acctschema_id:f64,
	/// primary
	/// not nullable 
	pub c_bp_group_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_prepayment_acct:f64,
	/// not nullable 
	pub c_receivable_acct:f64,
	pub c_receivable_services_acct:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub notinvoicedreceipts_acct:f64,
	/// not nullable 
	pub notinvoicedreceivables_acct:f64,
	/// not nullable 
	pub notinvoicedrevenue_acct:f64,
	/// not nullable 
	pub paydiscount_exp_acct:f64,
	/// not nullable 
	pub paydiscount_rev_acct:f64,
	pub processing:Option<String>,
	/// not nullable 
	pub unearnedrevenue_acct:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub v_liability_acct:f64,
	/// not nullable 
	pub v_liability_services_acct:f64,
	/// not nullable 
	pub v_prepayment_acct:f64,
	/// not nullable 
	pub writeoff_acct:f64,
	/// has one
	pub c_acctschema_id_c_acctschema:Option<CAcctschema>,
	/// has one
	pub c_bp_group_id_c_bp_group:Option<CBpGroup>,
	/// has one
	pub c_receivable_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub c_prepayment_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub v_liability_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub v_liability_services_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub v_prepayment_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub paydiscount_exp_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub paydiscount_rev_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub writeoff_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub notinvoicedreceipts_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub unearnedrevenue_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub notinvoicedrevenue_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub notinvoicedreceivables_acct_c_validcombination:Option<CValidcombination>,
}

pub struct CBpRelation {
	/// primary
	/// not nullable 
	pub c_bp_relation_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_bpartner_id:f64,
	pub c_bpartner_location_id:Option<f64>,
	/// not nullable 
	pub c_bpartnerrelation_id:f64,
	/// not nullable 
	pub c_bpartnerrelation_location_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isbillto:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub ispayfrom:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isremitto:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isshipto:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub c_bpartner_location_id_c_bpartner_location:Option<CBpartnerLocation>,
	/// has one
	pub c_bpartnerrelation_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub c_bpartnerrelation_location_id_c_bpartner_location:Option<CBpartnerLocation>,
}

pub struct CBpVendorAcct {
	/// primary
	/// not nullable 
	pub c_acctschema_id:f64,
	/// primary
	/// not nullable 
	pub c_bpartner_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub v_liability_acct:Option<f64>,
	pub v_liability_services_acct:Option<f64>,
	pub v_prepayment_acct:Option<f64>,
	/// has one
	pub c_acctschema_id_c_acctschema:Option<CAcctschema>,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub v_liability_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub v_liability_services_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub v_prepayment_acct_c_validcombination:Option<CValidcombination>,
}

pub struct CBpWithholding {
	/// primary
	/// not nullable 
	pub c_bpartner_id:f64,
	/// primary
	/// not nullable 
	pub c_withholding_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub exemptreason:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub ismandatorywithholding:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub istemporaryexempt:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub c_withholding_id_c_withholding:Option<CWithholding>,
}

pub struct CBpartner {
	/// primary
	/// not nullable 
	pub c_bpartner_id:f64,
	/// defaults to: 0
	pub acqusitioncost:Option<f64>,
	/// defaults to: 0
	pub actuallifetimevalue:Option<f64>,
	/// not nullable 
	pub ad_client_id:f64,
	pub ad_language:Option<String>,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_orgbp_id:Option<f64>,
	pub bpartner_parent_id:Option<f64>,
	/// not nullable 
	pub c_bp_group_id:f64,
	pub c_dunning_id:Option<f64>,
	pub c_greeting_id:Option<f64>,
	pub c_invoiceschedule_id:Option<f64>,
	pub c_paymentterm_id:Option<f64>,
	pub c_taxgroup_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub deliveryrule:Option<String>,
	pub deliveryviarule:Option<String>,
	pub description:Option<String>,
	pub documentcopies:Option<f64>,
	pub dunninggrace:Option<NaiveDate>,
	pub duns:Option<String>,
	/// defaults to: NULL::numeric
	pub edu_department_id:Option<f64>,
	pub firstsale:Option<NaiveDateTime>,
	pub flatdiscount:Option<f64>,
	pub freightcostrule:Option<String>,
	pub invoice_printformat_id:Option<f64>,
	pub invoicerule:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: NULL::bpchar
	pub iscashier:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub iscustomer:String,
	/// defaults to: NULL::bpchar
	pub isdean:Option<String>,
	/// defaults to: 'Y'::bpchar
	pub isdiscountprinted:Option<String>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isemployee:String,
	/// defaults to: NULL::bpchar
	pub isfaculty:Option<String>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isonetime:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub ispotaxexempt:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isprospect:String,
	/// defaults to: NULL::bpchar
	pub isregistrar:Option<String>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub issalesrep:String,
	pub isstudent:Option<String>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub issummary:String,
	/// defaults to: 'N'::bpchar
	pub istaxexempt:Option<String>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isvendor:String,
	/// defaults to: NULL::numeric
	pub logo_id:Option<f64>,
	pub m_discountschema_id:Option<f64>,
	pub m_pricelist_id:Option<f64>,
	pub naics:Option<String>,
	/// not nullable 
	pub name:String,
	pub name2:Option<String>,
	pub numberemployees:Option<f64>,
	pub paymentrule:Option<String>,
	pub paymentrulepo:Option<String>,
	pub po_discountschema_id:Option<f64>,
	pub po_paymentterm_id:Option<f64>,
	pub po_pricelist_id:Option<f64>,
	pub poreference:Option<String>,
	/// defaults to: 0
	pub potentiallifetimevalue:Option<f64>,
	pub rating:Option<String>,
	pub referenceno:Option<String>,
	pub salesrep_id:Option<f64>,
	pub salesvolume:Option<f64>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub sendemail:String,
	pub shareofcustomer:Option<f64>,
	pub shelflifeminpct:Option<f64>,
	/// defaults to: 0
	pub so_creditlimit:Option<f64>,
	/// defaults to: 0
	pub so_creditused:Option<f64>,
	pub so_description:Option<String>,
	/// defaults to: 'O'::bpchar
	pub socreditstatus:Option<String>,
	pub taxid:Option<String>,
	pub totalopenbalance:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub url:Option<String>,
	/// not nullable 
	pub value:String,
	/// has one
	pub ad_client_id_ad_client:Option<AdClient>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub c_bp_group_id_c_bp_group:Option<CBpGroup>,
	/// has one
	pub ad_language_ad_language:Option<AdLanguage>,
	/// has one
	pub c_invoiceschedule_id_c_invoiceschedule:Option<CInvoiceschedule>,
	/// has one
	pub c_paymentterm_id_c_paymentterm:Option<CPaymentterm>,
	/// has one
	pub m_pricelist_id_m_pricelist:Option<MPricelist>,
	/// has one
	pub m_discountschema_id_m_discountschema:Option<MDiscountschema>,
	/// has one
	pub c_dunning_id_c_dunning:Option<CDunning>,
	/// has one
	pub po_pricelist_id_m_pricelist:Option<MPricelist>,
	/// has one
	pub po_discountschema_id_m_discountschema:Option<MDiscountschema>,
	/// has one
	pub po_paymentterm_id_c_paymentterm:Option<CPaymentterm>,
	/// has one
	pub c_greeting_id_c_greeting:Option<CGreeting>,
	/// has one
	pub salesrep_id_ad_user:Option<AdUser>,
	/// has one, self referential
	pub bpartner_parent_id_c_bpartner:Option<Box<CBpartner>>,
	/// has one
	pub invoice_printformat_id_ad_printformat:Option<AdPrintformat>,
	/// has one
	pub ad_orgbp_id_ad_org:Option<AdOrg>,
	/// has one
	pub c_taxgroup_id_c_taxgroup:Option<CTaxgroup>,
	/// has many
	pub a_asset:Option<Vec<AAsset>>,
	/// has many
	pub a_registration:Option<Vec<ARegistration>>,
	/// has many
	pub ad_archive:Option<Vec<AdArchive>>,
	/// has many
	pub ad_clientinfo:Option<Vec<AdClientinfo>>,
	/// has many
	pub ad_user:Option<Vec<AdUser>>,
	/// has many
	pub ad_wf_node:Option<Vec<AdWfNode>>,
	/// has many
	pub c_acctschema_element:Option<Vec<CAcctschemaElement>>,
	/// has many
	pub c_allocationline:Option<Vec<CAllocationline>>,
	/// has many
	pub c_bankstatementline:Option<Vec<CBankstatementline>>,
	/// has many
	pub c_bp_bankaccount:Option<Vec<CBpBankaccount>>,
	/// has many
	pub c_bp_customer_acct:Option<Vec<CBpCustomerAcct>>,
	/// has many
	pub c_bp_edi:Option<Vec<CBpEdi>>,
	/// has many
	pub c_bp_employee_acct:Option<Vec<CBpEmployeeAcct>>,
	/// has many
	pub c_bp_relation:Option<Vec<CBpRelation>>,
	/// has many
	pub c_bp_vendor_acct:Option<Vec<CBpVendorAcct>>,
	/// has many
	pub c_bp_withholding:Option<Vec<CBpWithholding>>,
	/// has many
	pub c_bpartner:Option<Vec<CBpartner>>,
	/// has many
	pub c_bpartner_location:Option<Vec<CBpartnerLocation>>,
	/// has many
	pub c_bpartner_product:Option<Vec<CBpartnerProduct>>,
	/// has many
	pub c_charge:Option<Vec<CCharge>>,
	/// has many
	pub c_commission:Option<Vec<CCommission>>,
	/// has many
	pub c_commissionline:Option<Vec<CCommissionline>>,
	/// has many
	pub c_dunningrunentry:Option<Vec<CDunningrunentry>>,
	/// has many
	pub c_invoice:Option<Vec<CInvoice>>,
	/// has many
	pub c_invoicebatchline:Option<Vec<CInvoicebatchline>>,
	/// has many
	pub c_order:Option<Vec<COrder>>,
	/// has many
	pub c_orderline:Option<Vec<COrderline>>,
	/// has many
	pub c_payment:Option<Vec<CPayment>>,
	/// has many
	pub c_payselectioncheck:Option<Vec<CPayselectioncheck>>,
	/// has many
	pub c_pos:Option<Vec<CPos>>,
	/// has many
	pub c_project:Option<Vec<CProject>>,
	/// has many
	pub c_rfq:Option<Vec<CRfq>>,
	/// has many
	pub c_rfq_topicsubscriber:Option<Vec<CRfqTopicsubscriber>>,
	/// has many
	pub c_rfqresponse:Option<Vec<CRfqresponse>>,
	/// has many
	pub c_subscription:Option<Vec<CSubscription>>,
	/// has many
	pub c_taxdeclarationline:Option<Vec<CTaxdeclarationline>>,
	/// has many
	pub c_taxdefinition:Option<Vec<CTaxdefinition>>,
	/// has many
	pub c_validcombination:Option<Vec<CValidcombination>>,
	/// has many
	pub c_withholding:Option<Vec<CWithholding>>,
	/// has many
	pub dd_order:Option<Vec<DdOrder>>,
	/// has many
	pub fact_acct:Option<Vec<FactAcct>>,
	/// has many
	pub gl_distribution:Option<Vec<GlDistribution>>,
	/// has many
	pub gl_distributionline:Option<Vec<GlDistributionline>>,
	/// has many
	pub hr_attribute:Option<Vec<HrAttribute>>,
	/// has many
	pub hr_contract:Option<Vec<HrContract>>,
	/// has many
	pub hr_employee:Option<Vec<HrEmployee>>,
	/// has many
	pub hr_movement:Option<Vec<HrMovement>>,
	/// has many
	pub hr_process:Option<Vec<HrProcess>>,
	/// has many
	pub i_asset:Option<Vec<IAsset>>,
	/// has many
	pub i_bankstatement:Option<Vec<IBankstatement>>,
	/// has many
	pub i_bpartner:Option<Vec<IBpartner>>,
	/// has many
	pub i_fajournal:Option<Vec<IFajournal>>,
	/// has many
	pub i_gljournal:Option<Vec<IGljournal>>,
	/// has many
	pub i_invoice:Option<Vec<IInvoice>>,
	/// has many
	pub i_order:Option<Vec<IOrder>>,
	/// has many
	pub i_payment:Option<Vec<IPayment>>,
	/// has many
	pub i_product:Option<Vec<IProduct>>,
	/// has many
	pub m_discountschemaline:Option<Vec<MDiscountschemaline>>,
	/// has many
	pub m_distributionlistline:Option<Vec<MDistributionlistline>>,
	/// has many
	pub m_distributionrun:Option<Vec<MDistributionrun>>,
	/// has many
	pub m_inout:Option<Vec<MInout>>,
	/// has many
	pub m_movement:Option<Vec<MMovement>>,
	/// has many
	pub m_product_po:Option<Vec<MProductPo>>,
	/// has many
	pub m_rma:Option<Vec<MRma>>,
	/// has many
	pub m_shipper:Option<Vec<MShipper>>,
	/// has many
	pub pa_goalrestriction:Option<Vec<PaGoalrestriction>>,
	/// has many
	pub pa_reportcolumn:Option<Vec<PaReportcolumn>>,
	/// has many
	pub pa_reportsource:Option<Vec<PaReportsource>>,
	/// has many
	pub pa_sla_goal:Option<Vec<PaSlaGoal>>,
	/// has many
	pub pp_mrp:Option<Vec<PpMrp>>,
	/// has many
	pub pp_order_node:Option<Vec<PpOrderNode>>,
	/// has many
	pub r_request:Option<Vec<RRequest>>,
	/// has many
	pub r_requestaction:Option<Vec<RRequestaction>>,
	/// has many
	pub s_timeexpense:Option<Vec<STimeexpense>>,
	/// has many
	pub s_timeexpenseline:Option<Vec<STimeexpenseline>>,
	/// has many
	pub t_aging:Option<Vec<TAging>>,
	/// has many
	pub t_distributionrundetail:Option<Vec<TDistributionrundetail>>,
	/// has many
	pub w_advertisement:Option<Vec<WAdvertisement>>,
	/// has many
	pub w_basket:Option<Vec<WBasket>>,
	/// has many
	pub w_clickcount:Option<Vec<WClickcount>>,
	/// has many
	pub w_countercount:Option<Vec<WCountercount>>,
}

pub struct CBpartnerLocation {
	/// primary
	/// not nullable 
	pub c_bpartner_location_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_bpartner_id:f64,
	pub c_location_id:Option<f64>,
	pub c_salesregion_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub fax:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isbillto:String,
	pub isdn:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub ispayfrom:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isremitto:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isshipto:String,
	/// not nullable 
	pub name:String,
	pub phone:Option<String>,
	pub phone2:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_client_id_ad_client:Option<AdClient>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub c_salesregion_id_c_salesregion:Option<CSalesregion>,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub c_location_id_c_location:Option<CLocation>,
	/// has many
	pub a_asset:Option<Vec<AAsset>>,
	/// has many
	pub a_asset_change:Option<Vec<AAssetChange>>,
	/// has many
	pub ad_user:Option<Vec<AdUser>>,
	/// has many
	pub c_bp_relation:Option<Vec<CBpRelation>>,
	/// has many
	pub c_dunningrunentry:Option<Vec<CDunningrunentry>>,
	/// has many
	pub c_invoice:Option<Vec<CInvoice>>,
	/// has many
	pub c_invoicebatchline:Option<Vec<CInvoicebatchline>>,
	/// has many
	pub c_order:Option<Vec<COrder>>,
	/// has many
	pub c_orderline:Option<Vec<COrderline>>,
	/// has many
	pub c_project:Option<Vec<CProject>>,
	/// has many
	pub c_rfq:Option<Vec<CRfq>>,
	/// has many
	pub c_rfq_topicsubscriber:Option<Vec<CRfqTopicsubscriber>>,
	/// has many
	pub c_rfqresponse:Option<Vec<CRfqresponse>>,
	/// has many
	pub dd_order:Option<Vec<DdOrder>>,
	/// has many
	pub i_asset:Option<Vec<IAsset>>,
	/// has many
	pub i_bpartner:Option<Vec<IBpartner>>,
	/// has many
	pub i_invoice:Option<Vec<IInvoice>>,
	/// has many
	pub i_order:Option<Vec<IOrder>>,
	/// has many
	pub m_distributionlistline:Option<Vec<MDistributionlistline>>,
	/// has many
	pub m_distributionrun:Option<Vec<MDistributionrun>>,
	/// has many
	pub m_inout:Option<Vec<MInout>>,
	/// has many
	pub t_distributionrundetail:Option<Vec<TDistributionrundetail>>,
}

pub struct CBpartnerProduct {
	/// primary
	/// not nullable 
	pub c_bpartner_id:f64,
	/// primary
	/// not nullable 
	pub m_product_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub manufacturer:Option<String>,
	pub qualityrating:Option<f64>,
	/// not nullable 
	pub shelflifemindays:f64,
	/// not nullable 
	pub shelflifeminpct:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub vendorcategory:Option<String>,
	pub vendorproductno:Option<String>,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
}

pub struct CCalendar {
	/// primary
	/// not nullable 
	pub c_calendar_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_client_id_ad_client:Option<AdClient>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has many
	pub ad_clientinfo:Option<Vec<AdClientinfo>>,
	/// has many
	pub c_nonbusinessday:Option<Vec<CNonbusinessday>>,
	/// has many
	pub c_year:Option<Vec<CYear>>,
	/// has many
	pub m_demand:Option<Vec<MDemand>>,
	/// has many
	pub m_forecast:Option<Vec<MForecast>>,
	/// has many
	pub pa_report:Option<Vec<PaReport>>,
}

pub struct CCampaign {
	/// primary
	/// not nullable 
	pub c_campaign_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub c_channel_id:Option<f64>,
	/// defaults to: 0
	/// not nullable 
	pub costs:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub enddate:Option<NaiveDateTime>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub issummary:String,
	/// not nullable 
	pub name:String,
	pub startdate:Option<NaiveDateTime>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
	/// has one
	pub c_channel_id_c_channel:Option<CChannel>,
	/// has many
	pub c_acctschema_element:Option<Vec<CAcctschemaElement>>,
	/// has many
	pub c_cash:Option<Vec<CCash>>,
	/// has many
	pub c_invoice:Option<Vec<CInvoice>>,
	/// has many
	pub c_invoiceline:Option<Vec<CInvoiceline>>,
	/// has many
	pub c_order:Option<Vec<COrder>>,
	/// has many
	pub c_orderline:Option<Vec<COrderline>>,
	/// has many
	pub c_payment:Option<Vec<CPayment>>,
	/// has many
	pub c_project:Option<Vec<CProject>>,
	/// has many
	pub c_validcombination:Option<Vec<CValidcombination>>,
	/// has many
	pub dd_order:Option<Vec<DdOrder>>,
	/// has many
	pub dd_orderline:Option<Vec<DdOrderline>>,
	/// has many
	pub fact_acct:Option<Vec<FactAcct>>,
	/// has many
	pub gl_distribution:Option<Vec<GlDistribution>>,
	/// has many
	pub gl_distributionline:Option<Vec<GlDistributionline>>,
	/// has many
	pub hr_contract:Option<Vec<HrContract>>,
	/// has many
	pub hr_movement:Option<Vec<HrMovement>>,
	/// has many
	pub i_fajournal:Option<Vec<IFajournal>>,
	/// has many
	pub i_gljournal:Option<Vec<IGljournal>>,
	/// has many
	pub i_invoice:Option<Vec<IInvoice>>,
	/// has many
	pub i_order:Option<Vec<IOrder>>,
	/// has many
	pub m_inout:Option<Vec<MInout>>,
	/// has many
	pub m_inoutline:Option<Vec<MInoutline>>,
	/// has many
	pub m_inventory:Option<Vec<MInventory>>,
	/// has many
	pub m_movement:Option<Vec<MMovement>>,
	/// has many
	pub m_production:Option<Vec<MProduction>>,
	/// has many
	pub pa_reportcolumn:Option<Vec<PaReportcolumn>>,
	/// has many
	pub pa_reportsource:Option<Vec<PaReportsource>>,
	/// has many
	pub pp_cost_collector:Option<Vec<PpCostCollector>>,
	/// has many
	pub pp_order:Option<Vec<PpOrder>>,
	/// has many
	pub r_request:Option<Vec<RRequest>>,
	/// has many
	pub s_timeexpenseline:Option<Vec<STimeexpenseline>>,
	/// has many
	pub t_aging:Option<Vec<TAging>>,
}

pub struct CCash {
	/// primary
	/// not nullable 
	pub c_cash_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_orgtrx_id:Option<f64>,
	/// defaults to: 0
	/// not nullable 
	pub beginningbalance:f64,
	pub c_activity_id:Option<f64>,
	pub c_campaign_id:Option<f64>,
	/// not nullable 
	pub c_cashbook_id:f64,
	pub c_project_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub dateacct:NaiveDateTime,
	pub description:Option<String>,
	/// not nullable 
	pub docaction:String,
	/// not nullable 
	pub docstatus:String,
	/// defaults to: 0
	/// not nullable 
	pub endingbalance:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isapproved:String,
	/// not nullable 
	pub name:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub posted:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	pub processing:Option<String>,
	/// not nullable 
	pub statementdate:NaiveDateTime,
	/// defaults to: 0
	pub statementdifference:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub user1_id:Option<f64>,
	pub user2_id:Option<f64>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub c_cashbook_id_c_cashbook:Option<CCashbook>,
	/// has one
	pub ad_orgtrx_id_ad_org:Option<AdOrg>,
	/// has one
	pub c_project_id_c_project:Option<CProject>,
	/// has one
	pub c_campaign_id_c_campaign:Option<CCampaign>,
	/// has one
	pub c_activity_id_c_activity:Option<CActivity>,
	/// has one
	pub user1_id_c_elementvalue:Option<CElementvalue>,
	/// has one
	pub user2_id_c_elementvalue:Option<CElementvalue>,
	/// has many
	pub c_cashline:Option<Vec<CCashline>>,
}

pub struct CCashbook {
	/// primary
	/// not nullable 
	pub c_cashbook_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_currency_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isdefault:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_currency_id_c_currency:Option<CCurrency>,
	/// has many
	pub ad_orginfo:Option<Vec<AdOrginfo>>,
	/// has many
	pub c_cash:Option<Vec<CCash>>,
	/// has many
	pub c_cashbook_acct:Option<Vec<CCashbookAcct>>,
	/// has many
	pub c_payment:Option<Vec<CPayment>>,
	/// has many
	pub c_pos:Option<Vec<CPos>>,
}

pub struct CCashbookAcct {
	/// primary
	/// not nullable 
	pub c_cashbook_id:f64,
	/// primary
	/// not nullable 
	pub c_acctschema_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub cb_asset_acct:f64,
	/// not nullable 
	pub cb_cashtransfer_acct:f64,
	/// not nullable 
	pub cb_differences_acct:f64,
	/// not nullable 
	pub cb_expense_acct:f64,
	/// not nullable 
	pub cb_receipt_acct:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_cashbook_id_c_cashbook:Option<CCashbook>,
	/// has one
	pub c_acctschema_id_c_acctschema:Option<CAcctschema>,
	/// has one
	pub cb_asset_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub cb_cashtransfer_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub cb_differences_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub cb_expense_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub cb_receipt_acct_c_validcombination:Option<CValidcombination>,
}

pub struct CCashline {
	/// primary
	/// not nullable 
	pub c_cashline_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: 0
	/// not nullable 
	pub amount:f64,
	pub c_bankaccount_id:Option<f64>,
	/// not nullable 
	pub c_cash_id:f64,
	pub c_charge_id:Option<f64>,
	pub c_currency_id:Option<f64>,
	pub c_invoice_id:Option<f64>,
	pub c_payment_id:Option<f64>,
	/// not nullable 
	pub cashtype:String,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 0
	pub discountamt:Option<f64>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	pub isgenerated:Option<String>,
	/// not nullable 
	pub line:f64,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// defaults to: 0
	pub writeoffamt:Option<f64>,
	/// has one
	pub c_cash_id_c_cash:Option<CCash>,
	/// has one
	pub c_bankaccount_id_c_bankaccount:Option<CBankaccount>,
	/// has one
	pub c_charge_id_c_charge:Option<CCharge>,
	/// has one
	pub c_invoice_id_c_invoice:Option<CInvoice>,
	/// has one
	pub c_currency_id_c_currency:Option<CCurrency>,
	/// has one
	pub c_payment_id_c_payment:Option<CPayment>,
	/// has many
	pub c_allocationline:Option<Vec<CAllocationline>>,
	/// has many
	pub c_invoice:Option<Vec<CInvoice>>,
	/// has many
	pub c_order:Option<Vec<COrder>>,
}

pub struct CChannel {
	/// primary
	/// not nullable 
	pub c_channel_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_printcolor_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_printcolor_id_ad_printcolor:Option<AdPrintcolor>,
	/// has many
	pub c_campaign:Option<Vec<CCampaign>>,
}

pub struct CCharge {
	/// primary
	/// not nullable 
	pub c_charge_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub c_bpartner_id:Option<f64>,
	pub c_chargetype_id:Option<f64>,
	pub c_taxcategory_id:Option<f64>,
	/// defaults to: 0
	/// not nullable 
	pub chargeamt:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub issamecurrency:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub issametax:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub istaxincluded:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_taxcategory_id_c_taxcategory:Option<CTaxcategory>,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has many
	pub c_bankstatementline:Option<Vec<CBankstatementline>>,
	/// has many
	pub c_cashline:Option<Vec<CCashline>>,
	/// has many
	pub c_charge_acct:Option<Vec<CChargeAcct>>,
	/// has many
	pub c_commission:Option<Vec<CCommission>>,
	/// has many
	pub c_invoice:Option<Vec<CInvoice>>,
	/// has many
	pub c_invoicebatchline:Option<Vec<CInvoicebatchline>>,
	/// has many
	pub c_invoiceline:Option<Vec<CInvoiceline>>,
	/// has many
	pub c_order:Option<Vec<COrder>>,
	/// has many
	pub c_orderline:Option<Vec<COrderline>>,
	/// has many
	pub c_payment:Option<Vec<CPayment>>,
	/// has many
	pub dd_order:Option<Vec<DdOrder>>,
	/// has many
	pub dd_orderline:Option<Vec<DdOrderline>>,
	/// has many
	pub hr_payroll:Option<Vec<HrPayroll>>,
	/// has many
	pub hr_process:Option<Vec<HrProcess>>,
	/// has many
	pub i_bankstatement:Option<Vec<IBankstatement>>,
	/// has many
	pub i_invoice:Option<Vec<IInvoice>>,
	/// has many
	pub i_payment:Option<Vec<IPayment>>,
	/// has many
	pub m_inout:Option<Vec<MInout>>,
	/// has many
	pub m_inoutline:Option<Vec<MInoutline>>,
	/// has many
	pub m_inventoryline:Option<Vec<MInventoryline>>,
	/// has many
	pub m_movement:Option<Vec<MMovement>>,
	/// has many
	pub m_requisitionline:Option<Vec<MRequisitionline>>,
	/// has many
	pub m_rmaline:Option<Vec<MRmaline>>,
}

pub struct CChargeAcct {
	/// primary
	/// not nullable 
	pub c_charge_id:f64,
	/// primary
	/// not nullable 
	pub c_acctschema_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ch_expense_acct:f64,
	/// not nullable 
	pub ch_revenue_acct:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_charge_id_c_charge:Option<CCharge>,
	/// has one
	pub c_acctschema_id_c_acctschema:Option<CAcctschema>,
	/// has one
	pub ch_expense_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub ch_revenue_acct_c_validcombination:Option<CValidcombination>,
}

pub struct CChargeTrl {
	/// primary
	/// not nullable 
	pub ad_language:String,
	/// primary
	/// not nullable 
	pub c_charge_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub istranslated:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
}

pub struct CChargetype {
	/// primary
	/// not nullable 
	pub c_chargetype_id:f64,
	/// defaults to: NULL::numeric
	/// not nullable 
	pub ad_client_id:f64,
	/// defaults to: NULL::numeric
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
}

pub struct CChargetypeDoctype {
	/// primary
	/// not nullable 
	pub c_chargetype_id:f64,
	/// primary
	/// not nullable 
	pub c_doctype_id:f64,
	/// defaults to: NULL::numeric
	/// not nullable 
	pub ad_client_id:f64,
	/// defaults to: NULL::numeric
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isallownegative:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isallowpositive:String,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
}

pub struct CCity {
	/// primary
	/// not nullable 
	pub c_city_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub areacode:Option<String>,
	pub c_country_id:Option<f64>,
	pub c_region_id:Option<f64>,
	pub coordinates:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub locode:Option<String>,
	/// not nullable 
	pub name:String,
	pub postal:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_client_id_ad_client:Option<AdClient>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub c_country_id_c_country:Option<CCountry>,
	/// has one
	pub c_region_id_c_region:Option<CRegion>,
	/// has many
	pub c_location:Option<Vec<CLocation>>,
}

pub struct CCommission {
	/// primary
	/// not nullable 
	pub c_commission_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_bpartner_id:f64,
	/// not nullable 
	pub c_charge_id:f64,
	/// not nullable 
	pub c_currency_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub createfrom:Option<String>,
	pub datelastrun:Option<NaiveDateTime>,
	pub description:Option<String>,
	/// not nullable 
	pub docbasistype:String,
	/// not nullable 
	pub frequencytype:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub listdetails:String,
	/// not nullable 
	pub name:String,
	pub processing:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub c_currency_id_c_currency:Option<CCurrency>,
	/// has one
	pub c_charge_id_c_charge:Option<CCharge>,
	/// has many
	pub c_commissionline:Option<Vec<CCommissionline>>,
	/// has many
	pub c_commissionrun:Option<Vec<CCommissionrun>>,
}

pub struct CCommissionamt {
	/// primary
	/// not nullable 
	pub c_commissionamt_id:f64,
	/// defaults to: 0
	/// not nullable 
	pub actualqty:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_commissionline_id:f64,
	/// not nullable 
	pub c_commissionrun_id:f64,
	/// defaults to: 0
	/// not nullable 
	pub commissionamt:f64,
	/// defaults to: 0
	/// not nullable 
	pub convertedamt:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_commissionrun_id_c_commissionrun:Option<CCommissionrun>,
	/// has one
	pub c_commissionline_id_c_commissionline:Option<CCommissionline>,
	/// has many
	pub c_commissiondetail:Option<Vec<CCommissiondetail>>,
}

pub struct CCommissiondetail {
	/// primary
	/// not nullable 
	pub c_commissiondetail_id:f64,
	/// defaults to: 0
	/// not nullable 
	pub actualamt:f64,
	/// defaults to: 0
	/// not nullable 
	pub actualqty:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_commissionamt_id:f64,
	/// not nullable 
	pub c_currency_id:f64,
	pub c_invoiceline_id:Option<f64>,
	pub c_orderline_id:Option<f64>,
	/// defaults to: 0
	/// not nullable 
	pub convertedamt:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub info:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub reference:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_commissionamt_id_c_commissionamt:Option<CCommissionamt>,
	/// has one
	pub c_orderline_id_c_orderline:Option<COrderline>,
	/// has one
	pub c_invoiceline_id_c_invoiceline:Option<CInvoiceline>,
	/// has one
	pub c_currency_id_c_currency:Option<CCurrency>,
}

pub struct CCommissionline {
	/// primary
	/// not nullable 
	pub c_commissionline_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub amtmultiplier:f64,
	/// not nullable 
	pub amtsubtract:f64,
	pub c_bp_group_id:Option<f64>,
	pub c_bpartner_id:Option<f64>,
	/// not nullable 
	pub c_commission_id:f64,
	pub c_salesregion_id:Option<f64>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub commissionorders:String,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub ispositiveonly:String,
	/// not nullable 
	pub line:f64,
	pub m_product_category_id:Option<f64>,
	pub m_product_id:Option<f64>,
	pub org_id:Option<f64>,
	pub paymentrule:Option<String>,
	/// not nullable 
	pub qtymultiplier:f64,
	/// not nullable 
	pub qtysubtract:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_commission_id_c_commission:Option<CCommission>,
	/// has one
	pub org_id_ad_org:Option<AdOrg>,
	/// has one
	pub m_product_category_id_m_product_category:Option<MProductCategory>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub c_bp_group_id_c_bp_group:Option<CBpGroup>,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub c_salesregion_id_c_salesregion:Option<CSalesregion>,
	/// has many
	pub c_commissionamt:Option<Vec<CCommissionamt>>,
}

pub struct CCommissionrun {
	/// primary
	/// not nullable 
	pub c_commissionrun_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_commission_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// not nullable 
	pub documentno:String,
	/// defaults to: 0
	/// not nullable 
	pub grandtotal:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	pub processing:Option<String>,
	/// not nullable 
	pub startdate:NaiveDateTime,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_commission_id_c_commission:Option<CCommission>,
	/// has many
	pub c_commissionamt:Option<Vec<CCommissionamt>>,
}

pub struct CConversionRate {
	/// primary
	/// not nullable 
	pub c_conversion_rate_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_conversiontype_id:f64,
	/// not nullable 
	pub c_currency_id:f64,
	/// not nullable 
	pub c_currency_id_to:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 0
	/// not nullable 
	pub dividerate:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 0
	/// not nullable 
	pub multiplyrate:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub validfrom:NaiveDateTime,
	pub validto:Option<NaiveDateTime>,
	/// has one
	pub ad_client_id_ad_client:Option<AdClient>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub c_currency_id_c_currency:Option<CCurrency>,
	/// has one
	pub c_currency_id_to_c_currency:Option<CCurrency>,
	/// has one
	pub c_conversiontype_id_c_conversiontype:Option<CConversiontype>,
	/// has many
	pub i_conversion_rate:Option<Vec<IConversionRate>>,
}

pub struct CConversiontype {
	/// primary
	/// not nullable 
	pub c_conversiontype_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isdefault:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
	/// has many
	pub c_conversion_rate:Option<Vec<CConversionRate>>,
	/// has many
	pub c_invoice:Option<Vec<CInvoice>>,
	/// has many
	pub c_invoicebatch:Option<Vec<CInvoicebatch>>,
	/// has many
	pub c_order:Option<Vec<COrder>>,
	/// has many
	pub c_payment:Option<Vec<CPayment>>,
	/// has many
	pub gl_journal:Option<Vec<GlJournal>>,
	/// has many
	pub gl_journalline:Option<Vec<GlJournalline>>,
	/// has many
	pub i_conversion_rate:Option<Vec<IConversionRate>>,
	/// has many
	pub i_gljournal:Option<Vec<IGljournal>>,
	/// has many
	pub m_discountschemaline:Option<Vec<MDiscountschemaline>>,
	/// has many
	pub t_invoicegl:Option<Vec<TInvoicegl>>,
}

pub struct CCountry {
	/// primary
	/// not nullable 
	pub c_country_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	pub ad_language:Option<String>,
	/// not nullable 
	pub ad_org_id:f64,
	pub c_currency_id:Option<f64>,
	/// not nullable 
	pub countrycode:String,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// not nullable 
	pub displaysequence:String,
	pub displaysequencelocal:Option<String>,
	pub expressionbankaccountno:Option<String>,
	pub expressionbankroutingno:Option<String>,
	pub expressionphone:Option<String>,
	pub expressionpostal:Option<String>,
	pub expressionpostal_add:Option<String>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub haspostal_add:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub hasregion:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isaddresslineslocalreverse:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isaddresslinesreverse:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub ispostcodelookup:String,
	pub lookupclassname:Option<String>,
	pub lookupclientid:Option<String>,
	pub lookuppassword:Option<String>,
	pub lookupurl:Option<String>,
	pub mediasize:Option<String>,
	/// not nullable 
	pub name:String,
	pub regionname:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one, self referential
	pub c_country_id_c_country:Option<Box<CCountry>>,
	/// has one
	pub ad_client_id_ad_client:Option<AdClient>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub ad_language_ad_language:Option<AdLanguage>,
	/// has one
	pub c_currency_id_c_currency:Option<CCurrency>,
	/// has many
	pub c_city:Option<Vec<CCity>>,
	/// has many
	pub c_country:Option<Vec<CCountry>>,
	/// has many
	pub c_country_trl:Option<Vec<CCountryTrl>>,
	/// has many
	pub c_location:Option<Vec<CLocation>>,
	/// has many
	pub c_region:Option<Vec<CRegion>>,
	/// has many
	pub c_tax:Option<Vec<CTax>>,
	/// has many
	pub i_bpartner:Option<Vec<IBpartner>>,
	/// has many
	pub i_invoice:Option<Vec<IInvoice>>,
	/// has many
	pub i_order:Option<Vec<IOrder>>,
	/// has many
	pub m_freight:Option<Vec<MFreight>>,
}

pub struct CCountryTrl {
	/// primary
	/// not nullable 
	pub c_country_id:f64,
	/// primary
	/// not nullable 
	pub ad_language:String,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub istranslated:String,
	/// not nullable 
	pub name:String,
	pub regionname:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_country_id_c_country:Option<CCountry>,
	/// has one
	pub ad_language_ad_language:Option<AdLanguage>,
}

pub struct CCurrency {
	/// primary
	/// not nullable 
	pub c_currency_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub costingprecision:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub cursymbol:Option<String>,
	/// not nullable 
	pub description:String,
	pub emuentrydate:Option<NaiveDateTime>,
	/// defaults to: 0
	pub emurate:Option<f64>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isemumember:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub iseuro:String,
	/// not nullable 
	pub iso_code:String,
	pub roundofffactor:Option<f64>,
	/// not nullable 
	pub stdprecision:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_client_id_ad_client:Option<AdClient>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has many
	pub a_asset_reval_entry:Option<Vec<AAssetRevalEntry>>,
	/// has many
	pub a_depreciation_entry:Option<Vec<ADepreciationEntry>>,
	/// has many
	pub ad_registration:Option<Vec<AdRegistration>>,
	/// has many
	pub ad_role:Option<Vec<AdRole>>,
	/// has many
	pub c_acctschema:Option<Vec<CAcctschema>>,
	/// has many
	pub c_allocationhdr:Option<Vec<CAllocationhdr>>,
	/// has many
	pub c_bankaccount:Option<Vec<CBankaccount>>,
	/// has many
	pub c_bankstatementline:Option<Vec<CBankstatementline>>,
	/// has many
	pub c_cashbook:Option<Vec<CCashbook>>,
	/// has many
	pub c_cashline:Option<Vec<CCashline>>,
	/// has many
	pub c_commission:Option<Vec<CCommission>>,
	/// has many
	pub c_commissiondetail:Option<Vec<CCommissiondetail>>,
	/// has many
	pub c_conversion_rate:Option<Vec<CConversionRate>>,
	/// has many
	pub c_country:Option<Vec<CCountry>>,
	/// has many
	pub c_currency_acct:Option<Vec<CCurrencyAcct>>,
	/// has many
	pub c_currency_trl:Option<Vec<CCurrencyTrl>>,
	/// has many
	pub c_cycle:Option<Vec<CCycle>>,
	/// has many
	pub c_dunningrunentry:Option<Vec<CDunningrunentry>>,
	/// has many
	pub c_elementvalue:Option<Vec<CElementvalue>>,
	/// has many
	pub c_invoice:Option<Vec<CInvoice>>,
	/// has many
	pub c_invoicebatch:Option<Vec<CInvoicebatch>>,
	/// has many
	pub c_order:Option<Vec<COrder>>,
	/// has many
	pub c_orderline:Option<Vec<COrderline>>,
	/// has many
	pub c_payment:Option<Vec<CPayment>>,
	/// has many
	pub c_paymentprocessor:Option<Vec<CPaymentprocessor>>,
	/// has many
	pub c_project:Option<Vec<CProject>>,
	/// has many
	pub c_revenuerecognition_plan:Option<Vec<CRevenuerecognitionPlan>>,
	/// has many
	pub c_rfq:Option<Vec<CRfq>>,
	/// has many
	pub c_rfqresponse:Option<Vec<CRfqresponse>>,
	/// has many
	pub c_taxdeclarationline:Option<Vec<CTaxdeclarationline>>,
	/// has many
	pub fact_acct:Option<Vec<FactAcct>>,
	/// has many
	pub gl_journal:Option<Vec<GlJournal>>,
	/// has many
	pub gl_journalbatch:Option<Vec<GlJournalbatch>>,
	/// has many
	pub gl_journalline:Option<Vec<GlJournalline>>,
	/// has many
	pub i_bankstatement:Option<Vec<IBankstatement>>,
	/// has many
	pub i_conversion_rate:Option<Vec<IConversionRate>>,
	/// has many
	pub i_fajournal:Option<Vec<IFajournal>>,
	/// has many
	pub i_gljournal:Option<Vec<IGljournal>>,
	/// has many
	pub i_invoice:Option<Vec<IInvoice>>,
	/// has many
	pub i_order:Option<Vec<IOrder>>,
	/// has many
	pub i_payment:Option<Vec<IPayment>>,
	/// has many
	pub i_product:Option<Vec<IProduct>>,
	/// has many
	pub m_freight:Option<Vec<MFreight>>,
	/// has many
	pub m_pricelist:Option<Vec<MPricelist>>,
	/// has many
	pub m_product_po:Option<Vec<MProductPo>>,
	/// has many
	pub m_rma:Option<Vec<MRma>>,
	/// has many
	pub pa_reportcolumn:Option<Vec<PaReportcolumn>>,
	/// has many
	pub s_timeexpenseline:Option<Vec<STimeexpenseline>>,
	/// has many
	pub t_aging:Option<Vec<TAging>>,
	/// has many
	pub t_inventoryvalue:Option<Vec<TInventoryvalue>>,
}

pub struct CCurrencyAcct {
	/// primary
	/// not nullable 
	pub c_acctschema_id:f64,
	/// primary
	/// not nullable 
	pub c_currency_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub realizedgain_acct:f64,
	/// not nullable 
	pub realizedloss_acct:f64,
	/// not nullable 
	pub unrealizedgain_acct:f64,
	/// not nullable 
	pub unrealizedloss_acct:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_acctschema_id_c_acctschema:Option<CAcctschema>,
	/// has one
	pub c_currency_id_c_currency:Option<CCurrency>,
	/// has one
	pub unrealizedgain_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub unrealizedloss_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub realizedgain_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub realizedloss_acct_c_validcombination:Option<CValidcombination>,
}

pub struct CCurrencyTrl {
	/// primary
	/// not nullable 
	pub c_currency_id:f64,
	/// primary
	/// not nullable 
	pub ad_language:String,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub cursymbol:Option<String>,
	/// not nullable 
	pub description:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub istranslated:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_currency_id_c_currency:Option<CCurrency>,
	/// has one
	pub ad_language_ad_language:Option<AdLanguage>,
}

pub struct CCycle {
	/// primary
	/// not nullable 
	pub c_cycle_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_currency_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_currency_id_c_currency:Option<CCurrency>,
	/// has many
	pub c_cyclestep:Option<Vec<CCyclestep>>,
}

pub struct CCyclephase {
	/// primary
	/// not nullable 
	pub c_cyclestep_id:f64,
	/// primary
	/// not nullable 
	pub c_phase_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_cyclestep_id_c_cyclestep:Option<CCyclestep>,
	/// has one
	pub c_phase_id_c_phase:Option<CPhase>,
}

pub struct CCyclestep {
	/// primary
	/// not nullable 
	pub c_cyclestep_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_cycle_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub relativeweight:f64,
	/// not nullable 
	pub seqno:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_cycle_id_c_cycle:Option<CCycle>,
	/// has many
	pub c_cyclephase:Option<Vec<CCyclephase>>,
}

pub struct CDoctype {
	/// primary
	/// not nullable 
	pub c_doctype_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_printformat_id:Option<f64>,
	pub c_doctypedifference_id:Option<f64>,
	pub c_doctypeinvoice_id:Option<f64>,
	pub c_doctypeproforma_id:Option<f64>,
	pub c_doctypeshipment_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub definitesequence_id:Option<f64>,
	pub description:Option<String>,
	/// not nullable 
	pub docbasetype:String,
	pub docnosequence_id:Option<f64>,
	pub docsubtypeso:Option<String>,
	/// not nullable 
	pub documentcopies:f64,
	pub documentnote:Option<String>,
	/// not nullable 
	pub gl_category_id:f64,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub hascharges:String,
	/// defaults to: 'N'::bpchar
	pub hasproforma:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub iscreatecounter:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isdefault:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isdefaultcounterdoc:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isdocnocontrolled:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isindexed:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isintransit:String,
	/// defaults to: 'N'::bpchar
	pub isoverwritedateoncomplete:Option<String>,
	/// defaults to: 'N'::bpchar
	pub isoverwriteseqoncomplete:Option<String>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub ispickqaconfirm:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isshipconfirm:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub issotrx:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub issplitwhendifference:String,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub printname:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one, self referential
	pub c_doctypeproforma_id_c_doctype:Option<Box<CDoctype>>,
	/// has one, self referential
	pub c_doctypeshipment_id_c_doctype:Option<Box<CDoctype>>,
	/// has one, self referential
	pub c_doctypeinvoice_id_c_doctype:Option<Box<CDoctype>>,
	/// has one
	pub docnosequence_id_ad_sequence:Option<AdSequence>,
	/// has one
	pub gl_category_id_gl_category:Option<GlCategory>,
	/// has one
	pub ad_printformat_id_ad_printformat:Option<AdPrintformat>,
	/// has one, self referential
	pub c_doctypedifference_id_c_doctype:Option<Box<CDoctype>>,
	/// has one
	pub definitesequence_id_ad_sequence:Option<AdSequence>,
	/// has many
	pub a_asset_reval_entry:Option<Vec<AAssetRevalEntry>>,
	/// has many
	pub a_depreciation_entry:Option<Vec<ADepreciationEntry>>,
	/// has many
	pub ad_document_action_access:Option<Vec<AdDocumentActionAccess>>,
	/// has many
	pub ad_replicationdocument:Option<Vec<AdReplicationdocument>>,
	/// has many
	pub c_doctype:Option<Vec<CDoctype>>,
	/// has many
	pub c_doctype_trl:Option<Vec<CDoctypeTrl>>,
	/// has many
	pub c_doctypecounter:Option<Vec<CDoctypecounter>>,
	/// has many
	pub c_invoice:Option<Vec<CInvoice>>,
	/// has many
	pub c_invoicebatchline:Option<Vec<CInvoicebatchline>>,
	/// has many
	pub c_order:Option<Vec<COrder>>,
	/// has many
	pub c_payment:Option<Vec<CPayment>>,
	/// has many
	pub c_pos:Option<Vec<CPos>>,
	/// has many
	pub dd_order:Option<Vec<DdOrder>>,
	/// has many
	pub gl_distribution:Option<Vec<GlDistribution>>,
	/// has many
	pub gl_journal:Option<Vec<GlJournal>>,
	/// has many
	pub gl_journalbatch:Option<Vec<GlJournalbatch>>,
	/// has many
	pub hr_process:Option<Vec<HrProcess>>,
	/// has many
	pub i_fajournal:Option<Vec<IFajournal>>,
	/// has many
	pub i_gljournal:Option<Vec<IGljournal>>,
	/// has many
	pub i_invoice:Option<Vec<IInvoice>>,
	/// has many
	pub i_order:Option<Vec<IOrder>>,
	/// has many
	pub i_payment:Option<Vec<IPayment>>,
	/// has many
	pub k_index:Option<Vec<KIndex>>,
	/// has many
	pub k_indexstop:Option<Vec<KIndexstop>>,
	/// has many
	pub m_inout:Option<Vec<MInout>>,
	/// has many
	pub m_inventory:Option<Vec<MInventory>>,
	/// has many
	pub m_movement:Option<Vec<MMovement>>,
	/// has many
	pub m_requisition:Option<Vec<MRequisition>>,
	/// has many
	pub m_rma:Option<Vec<MRma>>,
	/// has many
	pub pp_cost_collector:Option<Vec<PpCostCollector>>,
	/// has many
	pub pp_order:Option<Vec<PpOrder>>,
	/// has many
	pub t_invoicegl:Option<Vec<TInvoicegl>>,
	/// has many
	pub t_replenish:Option<Vec<TReplenish>>,
}

pub struct CDoctypeTrl {
	/// primary
	/// not nullable 
	pub c_doctype_id:f64,
	/// primary
	/// not nullable 
	pub ad_language:String,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub documentnote:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub istranslated:String,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub printname:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_doctype_id_c_doctype:Option<CDoctype>,
	/// has one
	pub ad_language_ad_language:Option<AdLanguage>,
}

pub struct CDoctypecounter {
	/// primary
	/// not nullable 
	pub c_doctypecounter_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_doctype_id:f64,
	pub counter_c_doctype_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub docaction:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub iscreatecounter:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isvalid:String,
	/// not nullable 
	pub name:String,
	pub processing:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_doctype_id_c_doctype:Option<CDoctype>,
	/// has one
	pub counter_c_doctype_id_c_doctype:Option<CDoctype>,
}

pub struct CDunning {
	/// primary
	/// not nullable 
	pub c_dunning_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub createlevelssequentially:String,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isdefault:String,
	/// not nullable 
	pub name:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub senddunningletter:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has many
	pub c_bp_group:Option<Vec<CBpGroup>>,
	/// has many
	pub c_bpartner:Option<Vec<CBpartner>>,
	/// has many
	pub c_dunninglevel:Option<Vec<CDunninglevel>>,
}

pub struct CDunninglevel {
	/// primary
	/// not nullable 
	pub c_dunninglevel_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_dunning_id:f64,
	pub c_paymentterm_id:Option<f64>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub chargefee:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub chargeinterest:String,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub daysafterdue:f64,
	/// not nullable 
	pub daysbetweendunning:f64,
	pub description:Option<String>,
	pub dunning_printformat_id:Option<f64>,
	/// defaults to: 0
	pub feeamt:Option<f64>,
	/// defaults to: 0
	pub interestpercent:Option<f64>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub issetcreditstop:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub issetpaymentterm:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isshowalldue:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isshownotdue:String,
	/// defaults to: 'x'::character varying
	/// not nullable 
	pub name:String,
	pub note:Option<String>,
	/// not nullable 
	pub printname:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_dunning_id_c_dunning:Option<CDunning>,
	/// has one
	pub dunning_printformat_id_ad_printformat:Option<AdPrintformat>,
	/// has one
	pub c_paymentterm_id_c_paymentterm:Option<CPaymentterm>,
	/// has many
	pub c_dunninglevel_trl:Option<Vec<CDunninglevelTrl>>,
	/// has many
	pub c_dunningrun:Option<Vec<CDunningrun>>,
	/// has many
	pub c_invoice:Option<Vec<CInvoice>>,
}

pub struct CDunninglevelTrl {
	/// primary
	/// not nullable 
	pub ad_language:String,
	/// primary
	/// not nullable 
	pub c_dunninglevel_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub istranslated:String,
	pub note:Option<String>,
	/// not nullable 
	pub printname:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_language_ad_language:Option<AdLanguage>,
	/// has one
	pub c_dunninglevel_id_c_dunninglevel:Option<CDunninglevel>,
}

pub struct CDunningrun {
	/// primary
	/// not nullable 
	pub c_dunningrun_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_dunninglevel_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// not nullable 
	pub dunningdate:NaiveDateTime,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	pub processing:Option<String>,
	pub sendit:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_dunninglevel_id_c_dunninglevel:Option<CDunninglevel>,
	/// has many
	pub c_dunningrunentry:Option<Vec<CDunningrunentry>>,
}

pub struct CDunningrunentry {
	/// primary
	/// not nullable 
	pub c_dunningrunentry_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_user_id:Option<f64>,
	/// defaults to: 0
	/// not nullable 
	pub amt:f64,
	/// not nullable 
	pub c_bpartner_id:f64,
	/// not nullable 
	pub c_bpartner_location_id:f64,
	/// not nullable 
	pub c_currency_id:f64,
	/// not nullable 
	pub c_dunningrun_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub note:Option<String>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	/// defaults to: 0
	/// not nullable 
	pub qty:f64,
	/// not nullable 
	pub salesrep_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub c_bpartner_location_id_c_bpartner_location:Option<CBpartnerLocation>,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
	/// has one
	pub c_dunningrun_id_c_dunningrun:Option<CDunningrun>,
	/// has one
	pub c_currency_id_c_currency:Option<CCurrency>,
	/// has one
	pub salesrep_id_ad_user:Option<AdUser>,
	/// has many
	pub c_dunningrunline:Option<Vec<CDunningrunline>>,
}

pub struct CDunningrunline {
	/// primary
	/// not nullable 
	pub c_dunningrunline_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: 0
	/// not nullable 
	pub amt:f64,
	/// not nullable 
	pub c_dunningrunentry_id:f64,
	pub c_invoice_id:Option<f64>,
	pub c_invoicepayschedule_id:Option<f64>,
	pub c_payment_id:Option<f64>,
	/// defaults to: 0
	/// not nullable 
	pub convertedamt:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 0
	/// not nullable 
	pub daysdue:f64,
	/// defaults to: 0
	/// not nullable 
	pub feeamt:f64,
	/// defaults to: 0
	/// not nullable 
	pub interestamt:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isindispute:String,
	/// not nullable 
	pub openamt:f64,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	/// defaults to: 0
	/// not nullable 
	pub timesdunned:f64,
	/// defaults to: 0
	/// not nullable 
	pub totalamt:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_dunningrunentry_id_c_dunningrunentry:Option<CDunningrunentry>,
	/// has one
	pub c_invoice_id_c_invoice:Option<CInvoice>,
	/// has one
	pub c_payment_id_c_payment:Option<CPayment>,
	/// has one
	pub c_invoicepayschedule_id_c_invoicepayschedule:Option<CInvoicepayschedule>,
}

pub struct CElement {
	/// primary
	/// not nullable 
	pub c_element_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_tree_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// not nullable 
	pub elementtype:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isbalancing:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isnaturalaccount:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub vformat:Option<String>,
	/// has one
	pub ad_client_id_ad_client:Option<AdClient>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub ad_tree_id_ad_tree:Option<AdTree>,
	/// has many
	pub c_acctschema_element:Option<Vec<CAcctschemaElement>>,
	/// has many
	pub c_elementvalue:Option<Vec<CElementvalue>>,
	/// has many
	pub i_elementvalue:Option<Vec<IElementvalue>>,
}

pub struct CElementvalue {
	/// primary
	/// not nullable 
	pub c_elementvalue_id:f64,
	/// not nullable 
	pub accountsign:String,
	/// not nullable 
	pub accounttype:String,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub c_bankaccount_id:Option<f64>,
	pub c_currency_id:Option<f64>,
	/// not nullable 
	pub c_element_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	pub isbankaccount:Option<String>,
	/// defaults to: 'N'::bpchar
	pub isdoccontrolled:Option<String>,
	/// defaults to: 'N'::bpchar
	pub isforeigncurrency:Option<String>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub issummary:String,
	/// not nullable 
	pub name:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub postactual:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub postbudget:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub postencumbrance:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub poststatistical:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub validfrom:Option<NaiveDateTime>,
	pub validto:Option<NaiveDateTime>,
	/// not nullable 
	pub value:String,
	/// has one
	pub ad_client_id_ad_client:Option<AdClient>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub c_element_id_c_element:Option<CElement>,
	/// has one
	pub c_bankaccount_id_c_bankaccount:Option<CBankaccount>,
	/// has one
	pub c_currency_id_c_currency:Option<CCurrency>,
	/// has many
	pub c_acctschema_element:Option<Vec<CAcctschemaElement>>,
	/// has many
	pub c_cash:Option<Vec<CCash>>,
	/// has many
	pub c_elementvalue_trl:Option<Vec<CElementvalueTrl>>,
	/// has many
	pub c_invoice:Option<Vec<CInvoice>>,
	/// has many
	pub c_invoicebatchline:Option<Vec<CInvoicebatchline>>,
	/// has many
	pub c_invoiceline:Option<Vec<CInvoiceline>>,
	/// has many
	pub c_order:Option<Vec<COrder>>,
	/// has many
	pub c_orderline:Option<Vec<COrderline>>,
	/// has many
	pub c_payment:Option<Vec<CPayment>>,
	/// has many
	pub c_subacct:Option<Vec<CSubacct>>,
	/// has many
	pub c_validcombination:Option<Vec<CValidcombination>>,
	/// has many
	pub dd_order:Option<Vec<DdOrder>>,
	/// has many
	pub dd_orderline:Option<Vec<DdOrderline>>,
	/// has many
	pub fact_acct:Option<Vec<FactAcct>>,
	/// has many
	pub gl_distribution:Option<Vec<GlDistribution>>,
	/// has many
	pub gl_distributionline:Option<Vec<GlDistributionline>>,
	/// has many
	pub gl_fundrestriction:Option<Vec<GlFundrestriction>>,
	/// has many
	pub hr_concept_acct:Option<Vec<HrConceptAcct>>,
	/// has many
	pub hr_movement:Option<Vec<HrMovement>>,
	/// has many
	pub i_elementvalue:Option<Vec<IElementvalue>>,
	/// has many
	pub i_fajournal:Option<Vec<IFajournal>>,
	/// has many
	pub i_gljournal:Option<Vec<IGljournal>>,
	/// has many
	pub i_reportline:Option<Vec<IReportline>>,
	/// has many
	pub m_inout:Option<Vec<MInout>>,
	/// has many
	pub m_inoutline:Option<Vec<MInoutline>>,
	/// has many
	pub m_inventory:Option<Vec<MInventory>>,
	/// has many
	pub m_movement:Option<Vec<MMovement>>,
	/// has many
	pub m_production:Option<Vec<MProduction>>,
	/// has many
	pub pa_ratioelement:Option<Vec<PaRatioelement>>,
	/// has many
	pub pa_reportcolumn:Option<Vec<PaReportcolumn>>,
	/// has many
	pub pa_reportsource:Option<Vec<PaReportsource>>,
	/// has many
	pub pp_order:Option<Vec<PpOrder>>,
}

pub struct CElementvalueTrl {
	/// primary
	/// not nullable 
	pub c_elementvalue_id:f64,
	/// primary
	/// not nullable 
	pub ad_language:String,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub istranslated:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_elementvalue_id_c_elementvalue:Option<CElementvalue>,
	/// has one
	pub ad_language_ad_language:Option<AdLanguage>,
}

pub struct CGreeting {
	/// primary
	/// not nullable 
	pub c_greeting_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub greeting:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isdefault:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isfirstnameonly:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has many
	pub ad_user:Option<Vec<AdUser>>,
	/// has many
	pub c_bpartner:Option<Vec<CBpartner>>,
	/// has many
	pub c_greeting_trl:Option<Vec<CGreetingTrl>>,
	/// has many
	pub i_bpartner:Option<Vec<IBpartner>>,
}

pub struct CGreetingTrl {
	/// primary
	/// not nullable 
	pub ad_language:String,
	/// primary
	/// not nullable 
	pub c_greeting_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub greeting:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub istranslated:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_language_ad_language:Option<AdLanguage>,
	/// has one
	pub c_greeting_id_c_greeting:Option<CGreeting>,
}

pub struct CInterorgAcct {
	/// primary
	/// not nullable 
	pub c_acctschema_id:f64,
	/// primary
	/// not nullable 
	pub ad_org_id:f64,
	/// primary
	/// not nullable 
	pub ad_orgto_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub intercompanyduefrom_acct:f64,
	/// not nullable 
	pub intercompanydueto_acct:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_acctschema_id_c_acctschema:Option<CAcctschema>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub ad_orgto_id_ad_org:Option<AdOrg>,
	/// has one
	pub intercompanydueto_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub intercompanyduefrom_acct_c_validcombination:Option<CValidcombination>,
}

pub struct CInvoice {
	/// primary
	/// not nullable 
	pub c_invoice_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_orgtrx_id:Option<f64>,
	pub ad_user_id:Option<f64>,
	pub c_activity_id:Option<f64>,
	/// not nullable 
	pub c_bpartner_id:f64,
	/// not nullable 
	pub c_bpartner_location_id:f64,
	pub c_campaign_id:Option<f64>,
	pub c_cashline_id:Option<f64>,
	pub c_charge_id:Option<f64>,
	pub c_conversiontype_id:Option<f64>,
	/// not nullable 
	pub c_currency_id:f64,
	/// not nullable 
	pub c_doctype_id:f64,
	/// not nullable 
	pub c_doctypetarget_id:f64,
	pub c_dunninglevel_id:Option<f64>,
	pub c_order_id:Option<f64>,
	pub c_payment_id:Option<f64>,
	/// not nullable 
	pub c_paymentterm_id:f64,
	pub c_project_id:Option<f64>,
	/// defaults to: 0
	pub chargeamt:Option<f64>,
	pub copyfrom:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub createfrom:Option<String>,
	/// not nullable 
	pub dateacct:NaiveDateTime,
	/// not nullable 
	pub dateinvoiced:NaiveDateTime,
	pub dateordered:Option<NaiveDateTime>,
	pub dateprinted:Option<NaiveDateTime>,
	pub description:Option<String>,
	/// not nullable 
	pub docaction:String,
	/// not nullable 
	pub docstatus:String,
	/// not nullable 
	pub documentno:String,
	pub dunninggrace:Option<NaiveDate>,
	pub generateto:Option<String>,
	/// defaults to: 0
	/// not nullable 
	pub grandtotal:f64,
	pub invoicecollectiontype:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isapproved:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isdiscountprinted:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isindispute:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub ispaid:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub ispayschedulevalid:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isprinted:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isselfservice:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub issotrx:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub istaxincluded:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub istransferred:String,
	/// not nullable 
	pub m_pricelist_id:f64,
	pub m_rma_id:Option<f64>,
	/// not nullable 
	pub paymentrule:String,
	pub poreference:Option<String>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub posted:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	pub processing:Option<String>,
	pub ref_invoice_id:Option<f64>,
	pub reversal_id:Option<f64>,
	pub salesrep_id:Option<f64>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub sendemail:String,
	/// defaults to: 0
	/// not nullable 
	pub totallines:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub user1_id:Option<f64>,
	pub user2_id:Option<f64>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub c_doctype_id_c_doctype:Option<CDoctype>,
	/// has one
	pub c_doctypetarget_id_c_doctype:Option<CDoctype>,
	/// has one
	pub c_order_id_c_order:Option<COrder>,
	/// has one
	pub salesrep_id_ad_user:Option<AdUser>,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub c_bpartner_location_id_c_bpartner_location:Option<CBpartnerLocation>,
	/// has one
	pub c_currency_id_c_currency:Option<CCurrency>,
	/// has one
	pub c_paymentterm_id_c_paymentterm:Option<CPaymentterm>,
	/// has one
	pub c_charge_id_c_charge:Option<CCharge>,
	/// has one
	pub m_pricelist_id_m_pricelist:Option<MPricelist>,
	/// has one
	pub c_campaign_id_c_campaign:Option<CCampaign>,
	/// has one
	pub c_project_id_c_project:Option<CProject>,
	/// has one
	pub c_activity_id_c_activity:Option<CActivity>,
	/// has one
	pub c_payment_id_c_payment:Option<CPayment>,
	/// has one
	pub c_cashline_id_c_cashline:Option<CCashline>,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
	/// has one
	pub ad_orgtrx_id_ad_org:Option<AdOrg>,
	/// has one
	pub user1_id_c_elementvalue:Option<CElementvalue>,
	/// has one
	pub user2_id_c_elementvalue:Option<CElementvalue>,
	/// has one
	pub c_conversiontype_id_c_conversiontype:Option<CConversiontype>,
	/// has one, self referential
	pub ref_invoice_id_c_invoice:Option<Box<CInvoice>>,
	/// has one
	pub m_rma_id_m_rma:Option<MRma>,
	/// has one
	pub c_dunninglevel_id_c_dunninglevel:Option<CDunninglevel>,
	/// has one, self referential
	pub reversal_id_c_invoice:Option<Box<CInvoice>>,
	/// has many
	pub a_asset_addition:Option<Vec<AAssetAddition>>,
	/// has many
	pub c_allocationline:Option<Vec<CAllocationline>>,
	/// has many
	pub c_bankstatementline:Option<Vec<CBankstatementline>>,
	/// has many
	pub c_cashline:Option<Vec<CCashline>>,
	/// has many
	pub c_dunningrunline:Option<Vec<CDunningrunline>>,
	/// has many
	pub c_invoice:Option<Vec<CInvoice>>,
	/// has many
	pub c_invoicebatchline:Option<Vec<CInvoicebatchline>>,
	/// has many
	pub c_invoiceline:Option<Vec<CInvoiceline>>,
	/// has many
	pub c_invoicepayschedule:Option<Vec<CInvoicepayschedule>>,
	/// has many
	pub c_invoicetax:Option<Vec<CInvoicetax>>,
	/// has many
	pub c_payment:Option<Vec<CPayment>>,
	/// has many
	pub c_paymentallocate:Option<Vec<CPaymentallocate>>,
	/// has many
	pub c_payselectionline:Option<Vec<CPayselectionline>>,
	/// has many
	pub c_recurring:Option<Vec<CRecurring>>,
	/// has many
	pub c_recurring_run:Option<Vec<CRecurringRun>>,
	/// has many
	pub c_taxdeclarationline:Option<Vec<CTaxdeclarationline>>,
	/// has many
	pub dd_order:Option<Vec<DdOrder>>,
	/// has many
	pub i_bankstatement:Option<Vec<IBankstatement>>,
	/// has many
	pub i_invoice:Option<Vec<IInvoice>>,
	/// has many
	pub i_payment:Option<Vec<IPayment>>,
	/// has many
	pub m_inout:Option<Vec<MInout>>,
	/// has many
	pub m_inoutconfirm:Option<Vec<MInoutconfirm>>,
	/// has many
	pub r_request:Option<Vec<RRequest>>,
	/// has many
	pub r_requestaction:Option<Vec<RRequestaction>>,
	/// has many
	pub t_invoicegl:Option<Vec<TInvoicegl>>,
}

pub struct CInvoicebatch {
	/// primary
	/// not nullable 
	pub c_invoicebatch_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub c_conversiontype_id:Option<f64>,
	/// not nullable 
	pub c_currency_id:f64,
	/// defaults to: 0
	/// not nullable 
	pub controlamt:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub datedoc:NaiveDateTime,
	pub description:Option<String>,
	/// defaults to: 0
	/// not nullable 
	pub documentamt:f64,
	/// not nullable 
	pub documentno:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub issotrx:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	pub processing:Option<String>,
	/// not nullable 
	pub salesrep_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub salesrep_id_ad_user:Option<AdUser>,
	/// has one
	pub c_currency_id_c_currency:Option<CCurrency>,
	/// has one
	pub c_conversiontype_id_c_conversiontype:Option<CConversiontype>,
	/// has many
	pub c_invoicebatchline:Option<Vec<CInvoicebatchline>>,
}

pub struct CInvoicebatchline {
	/// primary
	/// not nullable 
	pub c_invoicebatchline_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_orgtrx_id:Option<f64>,
	pub ad_user_id:Option<f64>,
	pub c_activity_id:Option<f64>,
	/// not nullable 
	pub c_bpartner_id:f64,
	/// not nullable 
	pub c_bpartner_location_id:f64,
	/// not nullable 
	pub c_charge_id:f64,
	/// not nullable 
	pub c_doctype_id:f64,
	pub c_invoice_id:Option<f64>,
	/// not nullable 
	pub c_invoicebatch_id:f64,
	pub c_invoiceline_id:Option<f64>,
	pub c_project_id:Option<f64>,
	/// not nullable 
	pub c_tax_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub dateacct:NaiveDateTime,
	/// not nullable 
	pub dateinvoiced:NaiveDateTime,
	pub description:Option<String>,
	/// not nullable 
	pub documentno:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub istaxincluded:String,
	/// defaults to: 0
	/// not nullable 
	pub line:f64,
	/// defaults to: 0
	/// not nullable 
	pub linenetamt:f64,
	/// defaults to: 0
	/// not nullable 
	pub linetotalamt:f64,
	/// defaults to: 0
	/// not nullable 
	pub priceentered:f64,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	/// defaults to: 0
	/// not nullable 
	pub qtyentered:f64,
	/// defaults to: 0
	/// not nullable 
	pub taxamt:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub user1_id:Option<f64>,
	pub user2_id:Option<f64>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub c_invoicebatch_id_c_invoicebatch:Option<CInvoicebatch>,
	/// has one
	pub c_doctype_id_c_doctype:Option<CDoctype>,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub c_bpartner_location_id_c_bpartner_location:Option<CBpartnerLocation>,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
	/// has one
	pub c_charge_id_c_charge:Option<CCharge>,
	/// has one
	pub c_tax_id_c_tax:Option<CTax>,
	/// has one
	pub c_project_id_c_project:Option<CProject>,
	/// has one
	pub c_activity_id_c_activity:Option<CActivity>,
	/// has one
	pub ad_orgtrx_id_ad_org:Option<AdOrg>,
	/// has one
	pub user1_id_c_elementvalue:Option<CElementvalue>,
	/// has one
	pub user2_id_c_elementvalue:Option<CElementvalue>,
	/// has one
	pub c_invoice_id_c_invoice:Option<CInvoice>,
	/// has one
	pub c_invoiceline_id_c_invoiceline:Option<CInvoiceline>,
}

pub struct CInvoiceline {
	/// primary
	/// not nullable 
	pub c_invoiceline_id:f64,
	pub a_asset_group_id:Option<f64>,
	pub a_asset_id:Option<f64>,
	pub a_capvsexp:Option<String>,
	/// defaults to: 'N'::bpchar
	pub a_createasset:Option<String>,
	/// defaults to: 'N'::bpchar
	pub a_processed:Option<String>,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_orgtrx_id:Option<f64>,
	pub c_activity_id:Option<f64>,
	pub c_campaign_id:Option<f64>,
	pub c_charge_id:Option<f64>,
	/// not nullable 
	pub c_invoice_id:f64,
	pub c_orderline_id:Option<f64>,
	pub c_project_id:Option<f64>,
	pub c_projectphase_id:Option<f64>,
	pub c_projecttask_id:Option<f64>,
	pub c_tax_id:Option<f64>,
	pub c_uom_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isdescription:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isprinted:String,
	/// not nullable 
	pub line:f64,
	/// defaults to: 0
	/// not nullable 
	pub linenetamt:f64,
	/// defaults to: 0
	pub linetotalamt:Option<f64>,
	/// defaults to: 0
	pub m_attributesetinstance_id:Option<f64>,
	pub m_inoutline_id:Option<f64>,
	pub m_product_id:Option<f64>,
	pub m_rmaline_id:Option<f64>,
	/// defaults to: 0
	/// not nullable 
	pub priceactual:f64,
	/// not nullable 
	pub priceentered:f64,
	/// defaults to: 0
	/// not nullable 
	pub pricelimit:f64,
	/// defaults to: 0
	/// not nullable 
	pub pricelist:f64,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	/// not nullable 
	pub qtyentered:f64,
	/// defaults to: 0
	/// not nullable 
	pub qtyinvoiced:f64,
	pub ref_invoiceline_id:Option<f64>,
	pub rramt:Option<f64>,
	pub rrstartdate:Option<NaiveDateTime>,
	pub s_resourceassignment_id:Option<f64>,
	/// defaults to: 0
	pub taxamt:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub user1_id:Option<f64>,
	pub user2_id:Option<f64>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub c_invoice_id_c_invoice:Option<CInvoice>,
	/// has one
	pub c_orderline_id_c_orderline:Option<COrderline>,
	/// has one
	pub m_inoutline_id_m_inoutline:Option<MInoutline>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub c_charge_id_c_charge:Option<CCharge>,
	/// has one
	pub c_uom_id_c_uom:Option<CUom>,
	/// has one
	pub c_tax_id_c_tax:Option<CTax>,
	/// has one
	pub s_resourceassignment_id_s_resourceassignment:Option<SResourceassignment>,
	/// has one
	pub a_asset_id_a_asset:Option<AAsset>,
	/// has one
	pub m_attributesetinstance_id_m_attributesetinstance:Option<MAttributesetinstance>,
	/// has one, self referential
	pub ref_invoiceline_id_c_invoiceline:Option<Box<CInvoiceline>>,
	/// has one
	pub c_project_id_c_project:Option<CProject>,
	/// has one
	pub c_projectphase_id_c_projectphase:Option<CProjectphase>,
	/// has one
	pub c_projecttask_id_c_projecttask:Option<CProjecttask>,
	/// has one
	pub c_campaign_id_c_campaign:Option<CCampaign>,
	/// has one
	pub c_activity_id_c_activity:Option<CActivity>,
	/// has one
	pub user1_id_c_elementvalue:Option<CElementvalue>,
	/// has one
	pub user2_id_c_elementvalue:Option<CElementvalue>,
	/// has one
	pub ad_orgtrx_id_ad_org:Option<AdOrg>,
	/// has one
	pub m_rmaline_id_m_rmaline:Option<MRmaline>,
	/// has one
	pub a_asset_group_id_a_asset_group:Option<AAssetGroup>,
	/// has many
	pub a_asset_addition:Option<Vec<AAssetAddition>>,
	/// has many
	pub a_asset_retirement:Option<Vec<AAssetRetirement>>,
	/// has many
	pub c_commissiondetail:Option<Vec<CCommissiondetail>>,
	/// has many
	pub c_invoicebatchline:Option<Vec<CInvoicebatchline>>,
	/// has many
	pub c_invoiceline:Option<Vec<CInvoiceline>>,
	/// has many
	pub c_landedcost:Option<Vec<CLandedcost>>,
	/// has many
	pub c_landedcostallocation:Option<Vec<CLandedcostallocation>>,
	/// has many
	pub c_revenuerecognition_plan:Option<Vec<CRevenuerecognitionPlan>>,
	/// has many
	pub c_taxdeclarationline:Option<Vec<CTaxdeclarationline>>,
	/// has many
	pub i_invoice:Option<Vec<IInvoice>>,
	/// has many
	pub m_costdetail:Option<Vec<MCostdetail>>,
	/// has many
	pub m_inoutlineconfirm:Option<Vec<MInoutlineconfirm>>,
	/// has many
	pub m_matchinv:Option<Vec<MMatchinv>>,
	/// has many
	pub m_matchpo:Option<Vec<MMatchpo>>,
	/// has many
	pub s_timeexpenseline:Option<Vec<STimeexpenseline>>,
}

pub struct CInvoicepayschedule {
	/// primary
	/// not nullable 
	pub c_invoicepayschedule_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_invoice_id:f64,
	pub c_payschedule_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 0
	/// not nullable 
	pub discountamt:f64,
	/// not nullable 
	pub discountdate:NaiveDateTime,
	/// defaults to: 0
	/// not nullable 
	pub dueamt:f64,
	/// not nullable 
	pub duedate:NaiveDateTime,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isvalid:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	pub processing:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_invoice_id_c_invoice:Option<CInvoice>,
	/// has one
	pub c_payschedule_id_c_payschedule:Option<CPayschedule>,
	/// has many
	pub c_dunningrunline:Option<Vec<CDunningrunline>>,
}

pub struct CInvoiceschedule {
	/// primary
	/// not nullable 
	pub c_invoiceschedule_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: 0
	pub amt:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	pub eveninvoiceweek:Option<String>,
	pub invoiceday:Option<f64>,
	pub invoicedaycutoff:Option<f64>,
	/// not nullable 
	pub invoicefrequency:String,
	pub invoiceweekday:Option<String>,
	pub invoiceweekdaycutoff:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isamount:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isdefault:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has many
	pub c_bpartner:Option<Vec<CBpartner>>,
}

pub struct CInvoicetax {
	/// primary
	/// not nullable 
	pub c_tax_id:f64,
	/// primary
	/// not nullable 
	pub c_invoice_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub istaxincluded:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	/// defaults to: 0
	/// not nullable 
	pub taxamt:f64,
	/// defaults to: 0
	/// not nullable 
	pub taxbaseamt:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_tax_id_c_tax:Option<CTax>,
	/// has one
	pub c_invoice_id_c_invoice:Option<CInvoice>,
}

pub struct CJob {
	/// primary
	/// not nullable 
	pub c_job_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_jobcategory_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isemployee:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_jobcategory_id_c_jobcategory:Option<CJobcategory>,
	/// has many
	pub ad_user:Option<Vec<AdUser>>,
	/// has many
	pub c_jobassignment:Option<Vec<CJobassignment>>,
	/// has many
	pub c_jobremuneration:Option<Vec<CJobremuneration>>,
	/// has many
	pub m_operationresource:Option<Vec<MOperationresource>>,
}

pub struct CJobassignment {
	/// primary
	/// not nullable 
	pub c_jobassignment_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_user_id:f64,
	/// not nullable 
	pub c_job_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub validfrom:NaiveDateTime,
	pub validto:Option<NaiveDateTime>,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
	/// has one
	pub c_job_id_c_job:Option<CJob>,
}

pub struct CJobcategory {
	/// primary
	/// not nullable 
	pub c_jobcategory_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has many
	pub c_job:Option<Vec<CJob>>,
}

pub struct CJobremuneration {
	/// primary
	/// not nullable 
	pub c_jobremuneration_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_job_id:f64,
	/// not nullable 
	pub c_remuneration_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub validfrom:NaiveDateTime,
	pub validto:Option<NaiveDateTime>,
	/// has one
	pub c_job_id_c_job:Option<CJob>,
	/// has one
	pub c_remuneration_id_c_remuneration:Option<CRemuneration>,
}

pub struct CLandedcost {
	/// primary
	/// not nullable 
	pub c_landedcost_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_invoiceline_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// not nullable 
	pub landedcostdistribution:String,
	/// not nullable 
	pub m_costelement_id:f64,
	pub m_inout_id:Option<f64>,
	pub m_inoutline_id:Option<f64>,
	pub m_product_id:Option<f64>,
	pub processing:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_invoiceline_id_c_invoiceline:Option<CInvoiceline>,
	/// has one
	pub m_costelement_id_m_costelement:Option<MCostelement>,
	/// has one
	pub m_inoutline_id_m_inoutline:Option<MInoutline>,
	/// has one
	pub m_inout_id_m_inout:Option<MInout>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
}

pub struct CLandedcostallocation {
	/// primary
	/// not nullable 
	pub c_landedcostallocation_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: 0
	/// not nullable 
	pub amt:f64,
	/// defaults to: 0
	/// not nullable 
	pub base:f64,
	/// not nullable 
	pub c_invoiceline_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub m_attributesetinstance_id:Option<f64>,
	/// not nullable 
	pub m_costelement_id:f64,
	/// not nullable 
	pub m_product_id:f64,
	/// defaults to: 0
	/// not nullable 
	pub qty:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_invoiceline_id_c_invoiceline:Option<CInvoiceline>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub m_attributesetinstance_id_m_attributesetinstance:Option<MAttributesetinstance>,
	/// has one
	pub m_costelement_id_m_costelement:Option<MCostelement>,
}

pub struct CLocation {
	/// primary
	/// not nullable 
	pub c_location_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub address1:Option<String>,
	pub address2:Option<String>,
	pub address3:Option<String>,
	pub address4:Option<String>,
	pub c_city_id:Option<f64>,
	/// not nullable 
	pub c_country_id:f64,
	pub c_region_id:Option<f64>,
	pub city:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub postal:Option<String>,
	pub postal_add:Option<String>,
	pub regionname:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_client_id_ad_client:Option<AdClient>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub c_country_id_c_country:Option<CCountry>,
	/// has one
	pub c_region_id_c_region:Option<CRegion>,
	/// has one
	pub c_city_id_c_city:Option<CCity>,
	/// has many
	pub a_asset:Option<Vec<AAsset>>,
	/// has many
	pub a_asset_change:Option<Vec<AAssetChange>>,
	/// has many
	pub ad_orginfo:Option<Vec<AdOrginfo>>,
	/// has many
	pub c_acctschema_element:Option<Vec<CAcctschemaElement>>,
	/// has many
	pub c_bank:Option<Vec<CBank>>,
	/// has many
	pub c_bpartner_location:Option<Vec<CBpartnerLocation>>,
	/// has many
	pub c_validcombination:Option<Vec<CValidcombination>>,
	/// has many
	pub fact_acct:Option<Vec<FactAcct>>,
	/// has many
	pub gl_distribution:Option<Vec<GlDistribution>>,
	/// has many
	pub gl_distributionline:Option<Vec<GlDistributionline>>,
	/// has many
	pub i_asset:Option<Vec<IAsset>>,
	/// has many
	pub i_gljournal:Option<Vec<IGljournal>>,
	/// has many
	pub i_invoice:Option<Vec<IInvoice>>,
	/// has many
	pub i_order:Option<Vec<IOrder>>,
	/// has many
	pub m_warehouse:Option<Vec<MWarehouse>>,
	/// has many
	pub pa_reportcolumn:Option<Vec<PaReportcolumn>>,
	/// has many
	pub pa_reportsource:Option<Vec<PaReportsource>>,
}

pub struct CNonbusinessday {
	/// primary
	/// not nullable 
	pub c_nonbusinessday_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_calendar_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub date1:NaiveDateTime,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub name:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_client_id_ad_client:Option<AdClient>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub c_calendar_id_c_calendar:Option<CCalendar>,
}

pub struct COrder {
	/// primary
	/// not nullable 
	pub c_order_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_orgtrx_id:Option<f64>,
	pub ad_user_id:Option<f64>,
	pub amountrefunded:Option<f64>,
	pub amounttendered:Option<f64>,
	pub bill_bpartner_id:Option<f64>,
	pub bill_location_id:Option<f64>,
	pub bill_user_id:Option<f64>,
	pub c_activity_id:Option<f64>,
	/// not nullable 
	pub c_bpartner_id:f64,
	/// not nullable 
	pub c_bpartner_location_id:f64,
	pub c_campaign_id:Option<f64>,
	pub c_cashline_id:Option<f64>,
	pub c_charge_id:Option<f64>,
	pub c_conversiontype_id:Option<f64>,
	/// not nullable 
	pub c_currency_id:f64,
	/// not nullable 
	pub c_doctype_id:f64,
	/// not nullable 
	pub c_doctypetarget_id:f64,
	pub c_payment_id:Option<f64>,
	/// not nullable 
	pub c_paymentterm_id:f64,
	pub c_pos_id:Option<f64>,
	pub c_project_id:Option<f64>,
	/// defaults to: 0
	pub chargeamt:Option<f64>,
	pub copyfrom:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub dateacct:NaiveDateTime,
	/// not nullable 
	pub dateordered:NaiveDateTime,
	pub dateprinted:Option<NaiveDateTime>,
	pub datepromised:Option<NaiveDateTime>,
	/// not nullable 
	pub deliveryrule:String,
	/// not nullable 
	pub deliveryviarule:String,
	pub description:Option<String>,
	/// not nullable 
	pub docaction:String,
	/// not nullable 
	pub docstatus:String,
	/// not nullable 
	pub documentno:String,
	pub dropship_bpartner_id:Option<f64>,
	pub dropship_location_id:Option<f64>,
	pub dropship_user_id:Option<f64>,
	/// defaults to: 0
	pub freightamt:Option<f64>,
	/// not nullable 
	pub freightcostrule:String,
	/// defaults to: 0
	/// not nullable 
	pub grandtotal:f64,
	/// not nullable 
	pub invoicerule:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isapproved:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub iscreditapproved:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isdelivered:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isdiscountprinted:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isdropship:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isinvoiced:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isprinted:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isselected:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isselfservice:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub issotrx:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub istaxincluded:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub istransferred:String,
	pub link_order_id:Option<f64>,
	/// defaults to: NULL::numeric
	pub m_freightcategory_id:Option<f64>,
	/// not nullable 
	pub m_pricelist_id:f64,
	pub m_shipper_id:Option<f64>,
	/// not nullable 
	pub m_warehouse_id:f64,
	pub ordertype:Option<String>,
	pub pay_bpartner_id:Option<f64>,
	pub pay_location_id:Option<f64>,
	/// not nullable 
	pub paymentrule:String,
	pub poreference:Option<String>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub posted:String,
	/// not nullable 
	pub priorityrule:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	pub processing:Option<String>,
	pub promotioncode:Option<String>,
	pub ref_order_id:Option<f64>,
	pub salesrep_id:Option<f64>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub sendemail:String,
	/// defaults to: 0
	/// not nullable 
	pub totallines:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub user1_id:Option<f64>,
	pub user2_id:Option<f64>,
	pub volume:Option<f64>,
	pub weight:Option<f64>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub c_doctype_id_c_doctype:Option<CDoctype>,
	/// has one
	pub c_doctypetarget_id_c_doctype:Option<CDoctype>,
	/// has one
	pub salesrep_id_ad_user:Option<AdUser>,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub c_bpartner_location_id_c_bpartner_location:Option<CBpartnerLocation>,
	/// has one
	pub c_currency_id_c_currency:Option<CCurrency>,
	/// has one
	pub c_paymentterm_id_c_paymentterm:Option<CPaymentterm>,
	/// has one
	pub m_shipper_id_m_shipper:Option<MShipper>,
	/// has one
	pub c_charge_id_c_charge:Option<CCharge>,
	/// has one
	pub m_warehouse_id_m_warehouse:Option<MWarehouse>,
	/// has one
	pub m_pricelist_id_m_pricelist:Option<MPricelist>,
	/// has one
	pub c_campaign_id_c_campaign:Option<CCampaign>,
	/// has one
	pub c_project_id_c_project:Option<CProject>,
	/// has one
	pub c_activity_id_c_activity:Option<CActivity>,
	/// has one
	pub c_payment_id_c_payment:Option<CPayment>,
	/// has one
	pub c_cashline_id_c_cashline:Option<CCashline>,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
	/// has one
	pub ad_orgtrx_id_ad_org:Option<AdOrg>,
	/// has one
	pub user1_id_c_elementvalue:Option<CElementvalue>,
	/// has one
	pub user2_id_c_elementvalue:Option<CElementvalue>,
	/// has one
	pub c_conversiontype_id_c_conversiontype:Option<CConversiontype>,
	/// has one
	pub bill_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub bill_location_id_c_bpartner_location:Option<CBpartnerLocation>,
	/// has one
	pub bill_user_id_ad_user:Option<AdUser>,
	/// has one
	pub pay_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub pay_location_id_c_bpartner_location:Option<CBpartnerLocation>,
	/// has one, self referential
	pub ref_order_id_c_order:Option<Box<COrder>>,
	/// has one
	pub c_pos_id_c_pos:Option<CPos>,
	/// has one, self referential
	pub link_order_id_c_order:Option<Box<COrder>>,
	/// has one
	pub m_freightcategory_id_m_freightcategory:Option<MFreightcategory>,
	/// has many
	pub b_buyerfunds:Option<Vec<BBuyerfunds>>,
	/// has many
	pub b_sellerfunds:Option<Vec<BSellerfunds>>,
	/// has many
	pub c_allocationline:Option<Vec<CAllocationline>>,
	/// has many
	pub c_invoice:Option<Vec<CInvoice>>,
	/// has many
	pub c_order:Option<Vec<COrder>>,
	/// has many
	pub c_orderline:Option<Vec<COrderline>>,
	/// has many
	pub c_ordertax:Option<Vec<COrdertax>>,
	/// has many
	pub c_payment:Option<Vec<CPayment>>,
	/// has many
	pub c_projectline:Option<Vec<CProjectline>>,
	/// has many
	pub c_projectphase:Option<Vec<CProjectphase>>,
	/// has many
	pub c_recurring:Option<Vec<CRecurring>>,
	/// has many
	pub c_recurring_run:Option<Vec<CRecurringRun>>,
	/// has many
	pub c_rfq:Option<Vec<CRfq>>,
	/// has many
	pub c_rfqresponse:Option<Vec<CRfqresponse>>,
	/// has many
	pub dd_order:Option<Vec<DdOrder>>,
	/// has many
	pub i_order:Option<Vec<IOrder>>,
	/// has many
	pub m_inout:Option<Vec<MInout>>,
	/// has many
	pub m_rma:Option<Vec<MRma>>,
	/// has many
	pub pp_mrp:Option<Vec<PpMrp>>,
	/// has many
	pub r_request:Option<Vec<RRequest>>,
	/// has many
	pub r_requestaction:Option<Vec<RRequestaction>>,
}

pub struct COrderline {
	/// primary
	/// not nullable 
	pub c_orderline_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_orgtrx_id:Option<f64>,
	pub c_activity_id:Option<f64>,
	pub c_bpartner_id:Option<f64>,
	pub c_bpartner_location_id:Option<f64>,
	pub c_campaign_id:Option<f64>,
	pub c_charge_id:Option<f64>,
	/// not nullable 
	pub c_currency_id:f64,
	/// not nullable 
	pub c_order_id:f64,
	pub c_project_id:Option<f64>,
	pub c_projectphase_id:Option<f64>,
	pub c_projecttask_id:Option<f64>,
	/// not nullable 
	pub c_tax_id:f64,
	/// not nullable 
	pub c_uom_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub datedelivered:Option<NaiveDateTime>,
	pub dateinvoiced:Option<NaiveDateTime>,
	/// not nullable 
	pub dateordered:NaiveDateTime,
	pub datepromised:Option<NaiveDateTime>,
	pub description:Option<String>,
	pub discount:Option<f64>,
	/// defaults to: 0
	/// not nullable 
	pub freightamt:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isdescription:String,
	/// not nullable 
	pub line:f64,
	/// defaults to: 0
	/// not nullable 
	pub linenetamt:f64,
	pub link_orderline_id:Option<f64>,
	/// defaults to: 0
	pub m_attributesetinstance_id:Option<f64>,
	pub m_product_id:Option<f64>,
	pub m_promotion_id:Option<f64>,
	pub m_shipper_id:Option<f64>,
	/// not nullable 
	pub m_warehouse_id:f64,
	pub pp_cost_collector_id:Option<f64>,
	/// defaults to: 0
	/// not nullable 
	pub priceactual:f64,
	pub pricecost:Option<f64>,
	/// not nullable 
	pub priceentered:f64,
	/// defaults to: 0
	/// not nullable 
	pub pricelimit:f64,
	/// defaults to: 0
	/// not nullable 
	pub pricelist:f64,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	/// defaults to: 0
	/// not nullable 
	pub qtydelivered:f64,
	/// not nullable 
	pub qtyentered:f64,
	/// defaults to: 0
	/// not nullable 
	pub qtyinvoiced:f64,
	/// defaults to: 0
	/// not nullable 
	pub qtylostsales:f64,
	/// defaults to: 0
	/// not nullable 
	pub qtyordered:f64,
	/// defaults to: 0
	/// not nullable 
	pub qtyreserved:f64,
	pub ref_orderline_id:Option<f64>,
	pub rramt:Option<f64>,
	pub rrstartdate:Option<NaiveDateTime>,
	pub s_resourceassignment_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub user1_id:Option<f64>,
	pub user2_id:Option<f64>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub c_order_id_c_order:Option<COrder>,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub c_bpartner_location_id_c_bpartner_location:Option<CBpartnerLocation>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub m_warehouse_id_m_warehouse:Option<MWarehouse>,
	/// has one
	pub c_uom_id_c_uom:Option<CUom>,
	/// has one
	pub m_shipper_id_m_shipper:Option<MShipper>,
	/// has one
	pub c_currency_id_c_currency:Option<CCurrency>,
	/// has one
	pub c_charge_id_c_charge:Option<CCharge>,
	/// has one
	pub c_tax_id_c_tax:Option<CTax>,
	/// has one
	pub s_resourceassignment_id_s_resourceassignment:Option<SResourceassignment>,
	/// has one, self referential
	pub ref_orderline_id_c_orderline:Option<Box<COrderline>>,
	/// has one
	pub m_attributesetinstance_id_m_attributesetinstance:Option<MAttributesetinstance>,
	/// has one
	pub c_project_id_c_project:Option<CProject>,
	/// has one
	pub c_projectphase_id_c_projectphase:Option<CProjectphase>,
	/// has one
	pub c_projecttask_id_c_projecttask:Option<CProjecttask>,
	/// has one
	pub c_campaign_id_c_campaign:Option<CCampaign>,
	/// has one
	pub c_activity_id_c_activity:Option<CActivity>,
	/// has one
	pub user1_id_c_elementvalue:Option<CElementvalue>,
	/// has one
	pub user2_id_c_elementvalue:Option<CElementvalue>,
	/// has one
	pub ad_orgtrx_id_ad_org:Option<AdOrg>,
	/// has one, self referential
	pub link_orderline_id_c_orderline:Option<Box<COrderline>>,
	/// has many
	pub c_commissiondetail:Option<Vec<CCommissiondetail>>,
	/// has many
	pub c_invoiceline:Option<Vec<CInvoiceline>>,
	/// has many
	pub c_orderline:Option<Vec<COrderline>>,
	/// has many
	pub i_order:Option<Vec<IOrder>>,
	/// has many
	pub m_costdetail:Option<Vec<MCostdetail>>,
	/// has many
	pub m_demanddetail:Option<Vec<MDemanddetail>>,
	/// has many
	pub m_inoutline:Option<Vec<MInoutline>>,
	/// has many
	pub m_matchpo:Option<Vec<MMatchpo>>,
	/// has many
	pub m_requisitionline:Option<Vec<MRequisitionline>>,
	/// has many
	pub pp_mrp:Option<Vec<PpMrp>>,
	/// has many
	pub pp_order:Option<Vec<PpOrder>>,
	/// has many
	pub s_timeexpenseline:Option<Vec<STimeexpenseline>>,
}

pub struct COrdertax {
	/// primary
	/// not nullable 
	pub c_order_id:f64,
	/// primary
	/// not nullable 
	pub c_tax_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub istaxincluded:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	/// defaults to: 0
	/// not nullable 
	pub taxamt:f64,
	/// defaults to: 0
	/// not nullable 
	pub taxbaseamt:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_order_id_c_order:Option<COrder>,
	/// has one
	pub c_tax_id_c_tax:Option<CTax>,
}

pub struct COrgassignment {
	/// primary
	/// not nullable 
	pub c_orgassignment_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_user_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub validfrom:NaiveDateTime,
	/// not nullable 
	pub validto:NaiveDateTime,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
}

pub struct CPayment {
	/// primary
	/// not nullable 
	pub c_payment_id:f64,
	pub a_city:Option<String>,
	pub a_country:Option<String>,
	pub a_email:Option<String>,
	pub a_ident_dl:Option<String>,
	pub a_ident_ssn:Option<String>,
	pub a_name:Option<String>,
	pub a_state:Option<String>,
	pub a_street:Option<String>,
	pub a_zip:Option<String>,
	pub accountno:Option<String>,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_orgtrx_id:Option<f64>,
	pub c_activity_id:Option<f64>,
	pub c_bankaccount_id:Option<f64>,
	pub c_bp_bankaccount_id:Option<f64>,
	pub c_bpartner_id:Option<f64>,
	pub c_campaign_id:Option<f64>,
	pub c_cashbook_id:Option<f64>,
	pub c_charge_id:Option<f64>,
	pub c_conversiontype_id:Option<f64>,
	/// not nullable 
	pub c_currency_id:f64,
	/// not nullable 
	pub c_doctype_id:f64,
	pub c_invoice_id:Option<f64>,
	pub c_order_id:Option<f64>,
	pub c_paymentbatch_id:Option<f64>,
	pub c_project_id:Option<f64>,
	/// defaults to: 0
	pub chargeamt:Option<f64>,
	pub checkno:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub creditcardexpmm:Option<f64>,
	pub creditcardexpyy:Option<f64>,
	pub creditcardnumber:Option<String>,
	pub creditcardtype:Option<String>,
	pub creditcardvv:Option<String>,
	/// not nullable 
	pub dateacct:NaiveDateTime,
	/// not nullable 
	pub datetrx:NaiveDateTime,
	pub description:Option<String>,
	/// defaults to: 0
	pub discountamt:Option<f64>,
	/// not nullable 
	pub docaction:String,
	/// not nullable 
	pub docstatus:String,
	/// not nullable 
	pub documentno:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isallocated:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isapproved:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isdelayedcapture:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isonline:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isoverunderpayment:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isprepayment:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isreceipt:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isreconciled:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isselfservice:String,
	pub micr:Option<String>,
	pub oprocessing:Option<String>,
	pub orig_trxid:Option<String>,
	/// defaults to: 0
	pub overunderamt:Option<f64>,
	/// defaults to: 0
	/// not nullable 
	pub payamt:f64,
	pub ponum:Option<String>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub posted:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	pub processing:Option<String>,
	pub r_authcode:Option<String>,
	pub r_authcode_dc:Option<String>,
	pub r_avsaddr:Option<String>,
	pub r_avszip:Option<String>,
	pub r_cvv2match:Option<String>,
	pub r_info:Option<String>,
	pub r_pnref:Option<String>,
	pub r_pnref_dc:Option<String>,
	pub r_respmsg:Option<String>,
	pub r_result:Option<String>,
	pub ref_payment_id:Option<f64>,
	pub reversal_id:Option<f64>,
	pub routingno:Option<String>,
	pub swipe:Option<String>,
	/// defaults to: 0
	pub taxamt:Option<f64>,
	/// not nullable 
	pub tendertype:String,
	/// not nullable 
	pub trxtype:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub user1_id:Option<f64>,
	pub user2_id:Option<f64>,
	pub voiceauthcode:Option<String>,
	/// defaults to: 0
	pub writeoffamt:Option<f64>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub c_doctype_id_c_doctype:Option<CDoctype>,
	/// has one
	pub c_bankaccount_id_c_bankaccount:Option<CBankaccount>,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub c_invoice_id_c_invoice:Option<CInvoice>,
	/// has one
	pub c_bp_bankaccount_id_c_bp_bankaccount:Option<CBpBankaccount>,
	/// has one
	pub c_paymentbatch_id_c_paymentbatch:Option<CPaymentbatch>,
	/// has one
	pub c_currency_id_c_currency:Option<CCurrency>,
	/// has one
	pub c_project_id_c_project:Option<CProject>,
	/// has one
	pub c_charge_id_c_charge:Option<CCharge>,
	/// has one
	pub ad_orgtrx_id_ad_org:Option<AdOrg>,
	/// has one
	pub c_campaign_id_c_campaign:Option<CCampaign>,
	/// has one
	pub c_activity_id_c_activity:Option<CActivity>,
	/// has one
	pub user1_id_c_elementvalue:Option<CElementvalue>,
	/// has one
	pub user2_id_c_elementvalue:Option<CElementvalue>,
	/// has one
	pub c_conversiontype_id_c_conversiontype:Option<CConversiontype>,
	/// has one
	pub c_order_id_c_order:Option<COrder>,
	/// has one, self referential
	pub ref_payment_id_c_payment:Option<Box<CPayment>>,
	/// has one, self referential
	pub reversal_id_c_payment:Option<Box<CPayment>>,
	/// has one
	pub c_cashbook_id_c_cashbook:Option<CCashbook>,
	/// has many
	pub b_buyerfunds:Option<Vec<BBuyerfunds>>,
	/// has many
	pub b_sellerfunds:Option<Vec<BSellerfunds>>,
	/// has many
	pub c_allocationline:Option<Vec<CAllocationline>>,
	/// has many
	pub c_bankstatementline:Option<Vec<CBankstatementline>>,
	/// has many
	pub c_cashline:Option<Vec<CCashline>>,
	/// has many
	pub c_dunningrunline:Option<Vec<CDunningrunline>>,
	/// has many
	pub c_invoice:Option<Vec<CInvoice>>,
	/// has many
	pub c_order:Option<Vec<COrder>>,
	/// has many
	pub c_payment:Option<Vec<CPayment>>,
	/// has many
	pub c_paymentallocate:Option<Vec<CPaymentallocate>>,
	/// has many
	pub c_payselectioncheck:Option<Vec<CPayselectioncheck>>,
	/// has many
	pub c_recurring:Option<Vec<CRecurring>>,
	/// has many
	pub c_recurring_run:Option<Vec<CRecurringRun>>,
	/// has many
	pub i_bankstatement:Option<Vec<IBankstatement>>,
	/// has many
	pub i_payment:Option<Vec<IPayment>>,
	/// has many
	pub r_request:Option<Vec<RRequest>>,
	/// has many
	pub r_requestaction:Option<Vec<RRequestaction>>,
}

pub struct CPaymentallocate {
	/// primary
	/// not nullable 
	pub c_paymentallocate_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: 0
	/// not nullable 
	pub amount:f64,
	pub c_allocationline_id:Option<f64>,
	/// not nullable 
	pub c_invoice_id:f64,
	/// not nullable 
	pub c_payment_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 0
	/// not nullable 
	pub discountamt:f64,
	/// defaults to: 0
	pub invoiceamt:Option<f64>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 0
	/// not nullable 
	pub overunderamt:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// defaults to: 0
	/// not nullable 
	pub writeoffamt:f64,
	/// has one
	pub c_payment_id_c_payment:Option<CPayment>,
	/// has one
	pub c_invoice_id_c_invoice:Option<CInvoice>,
	/// has one
	pub c_allocationline_id_c_allocationline:Option<CAllocationline>,
}

pub struct CPaymentbatch {
	/// primary
	/// not nullable 
	pub c_paymentbatch_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub c_paymentprocessor_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub documentno:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	/// not nullable 
	pub processing:String,
	pub processingdate:Option<NaiveDateTime>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_paymentprocessor_id_c_paymentprocessor:Option<CPaymentprocessor>,
	/// has many
	pub c_payment:Option<Vec<CPayment>>,
}

pub struct CPaymentprocessor {
	/// primary
	/// not nullable 
	pub c_paymentprocessor_id:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub acceptamex:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub acceptatm:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub acceptcheck:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub acceptcorporate:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub acceptdiners:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub acceptdirectdebit:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub acceptdirectdeposit:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub acceptdiscover:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub acceptmc:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub acceptvisa:String,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_sequence_id:Option<f64>,
	/// not nullable 
	pub c_bankaccount_id:f64,
	pub c_currency_id:Option<f64>,
	/// not nullable 
	pub commission:f64,
	/// defaults to: 0
	/// not nullable 
	pub costpertrx:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub hostaddress:Option<String>,
	pub hostport:Option<f64>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 0
	pub minimumamt:Option<f64>,
	/// not nullable 
	pub name:String,
	pub partnerid:Option<String>,
	pub password:Option<String>,
	pub payprocessorclass:Option<String>,
	pub proxyaddress:Option<String>,
	pub proxylogon:Option<String>,
	pub proxypassword:Option<String>,
	pub proxyport:Option<f64>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub requirevv:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub userid:Option<String>,
	pub vendorid:Option<String>,
	/// has one
	pub c_bankaccount_id_c_bankaccount:Option<CBankaccount>,
	/// has one
	pub ad_sequence_id_ad_sequence:Option<AdSequence>,
	/// has one
	pub c_currency_id_c_currency:Option<CCurrency>,
	/// has many
	pub c_paymentbatch:Option<Vec<CPaymentbatch>>,
}

pub struct CPaymentterm {
	/// primary
	/// not nullable 
	pub c_paymentterm_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub afterdelivery:String,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// not nullable 
	pub discount:f64,
	/// not nullable 
	pub discount2:f64,
	/// not nullable 
	pub discountdays:f64,
	/// not nullable 
	pub discountdays2:f64,
	pub documentnote:Option<String>,
	pub fixmonthcutoff:Option<f64>,
	pub fixmonthday:Option<f64>,
	pub fixmonthoffset:Option<f64>,
	/// not nullable 
	pub gracedays:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	pub isdefault:Option<String>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isduefixed:String,
	/// defaults to: 'Y'::bpchar
	pub isnextbusinessday:Option<String>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isvalid:String,
	/// not nullable 
	pub name:String,
	pub netday:Option<String>,
	/// not nullable 
	pub netdays:f64,
	pub processing:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
	/// has many
	pub c_bpartner:Option<Vec<CBpartner>>,
	/// has many
	pub c_dunninglevel:Option<Vec<CDunninglevel>>,
	/// has many
	pub c_invoice:Option<Vec<CInvoice>>,
	/// has many
	pub c_order:Option<Vec<COrder>>,
	/// has many
	pub c_paymentterm_trl:Option<Vec<CPaymenttermTrl>>,
	/// has many
	pub c_payschedule:Option<Vec<CPayschedule>>,
	/// has many
	pub c_project:Option<Vec<CProject>>,
	/// has many
	pub c_withholding:Option<Vec<CWithholding>>,
	/// has many
	pub i_invoice:Option<Vec<IInvoice>>,
	/// has many
	pub i_order:Option<Vec<IOrder>>,
	/// has many
	pub w_store:Option<Vec<WStore>>,
}

pub struct CPaymenttermTrl {
	/// primary
	/// not nullable 
	pub c_paymentterm_id:f64,
	/// primary
	/// not nullable 
	pub ad_language:String,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub documentnote:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub istranslated:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_paymentterm_id_c_paymentterm:Option<CPaymentterm>,
	/// has one
	pub ad_language_ad_language:Option<AdLanguage>,
}

pub struct CPayschedule {
	/// primary
	/// not nullable 
	pub c_payschedule_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_paymentterm_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub discount:f64,
	/// not nullable 
	pub discountdays:f64,
	/// not nullable 
	pub gracedays:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isvalid:String,
	pub netday:Option<String>,
	/// not nullable 
	pub netdays:f64,
	/// not nullable 
	pub percentage:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_paymentterm_id_c_paymentterm:Option<CPaymentterm>,
	/// has many
	pub c_invoicepayschedule:Option<Vec<CInvoicepayschedule>>,
}

pub struct CPayselection {
	/// primary
	/// not nullable 
	pub c_payselection_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_bankaccount_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub createfrom:Option<String>,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isapproved:String,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub paydate:NaiveDateTime,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	pub processing:Option<String>,
	/// defaults to: 0
	/// not nullable 
	pub totalamt:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_bankaccount_id_c_bankaccount:Option<CBankaccount>,
	/// has many
	pub c_payselectioncheck:Option<Vec<CPayselectioncheck>>,
	/// has many
	pub c_payselectionline:Option<Vec<CPayselectionline>>,
	/// has many
	pub hr_process:Option<Vec<HrProcess>>,
}

pub struct CPayselectioncheck {
	/// primary
	/// not nullable 
	pub c_payselectioncheck_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub c_bp_bankaccount_id:Option<f64>,
	/// not nullable 
	pub c_bpartner_id:f64,
	pub c_payment_id:Option<f64>,
	/// not nullable 
	pub c_payselection_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 0
	/// not nullable 
	pub discountamt:f64,
	pub documentno:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isgenerateddraft:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isprinted:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isreceipt:String,
	/// defaults to: 0
	/// not nullable 
	pub payamt:f64,
	/// not nullable 
	pub paymentrule:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	/// defaults to: 0
	/// not nullable 
	pub qty:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_payselection_id_c_payselection:Option<CPayselection>,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub c_payment_id_c_payment:Option<CPayment>,
	/// has one
	pub c_bp_bankaccount_id_c_bp_bankaccount:Option<CBpBankaccount>,
	/// has many
	pub c_payselectionline:Option<Vec<CPayselectionline>>,
}

pub struct CPayselectionline {
	/// primary
	/// not nullable 
	pub c_payselectionline_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_invoice_id:f64,
	/// not nullable 
	pub c_payselection_id:f64,
	pub c_payselectioncheck_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 0
	/// not nullable 
	pub differenceamt:f64,
	/// defaults to: 0
	/// not nullable 
	pub discountamt:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub ismanual:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub issotrx:String,
	/// not nullable 
	pub line:f64,
	/// defaults to: 0
	/// not nullable 
	pub openamt:f64,
	/// defaults to: 0
	/// not nullable 
	pub payamt:f64,
	/// not nullable 
	pub paymentrule:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_payselection_id_c_payselection:Option<CPayselection>,
	/// has one
	pub c_invoice_id_c_invoice:Option<CInvoice>,
	/// has one
	pub c_payselectioncheck_id_c_payselectioncheck:Option<CPayselectioncheck>,
}

pub struct CPeriod {
	/// primary
	/// not nullable 
	pub c_period_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_year_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub enddate:Option<NaiveDateTime>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub periodno:f64,
	/// not nullable 
	pub periodtype:String,
	pub processing:Option<String>,
	/// not nullable 
	pub startdate:NaiveDateTime,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_client_id_ad_client:Option<AdClient>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub c_year_id_c_year:Option<CYear>,
	/// has many
	pub a_asset_disposed:Option<Vec<AAssetDisposed>>,
	/// has many
	pub a_asset_reval_entry:Option<Vec<AAssetRevalEntry>>,
	/// has many
	pub a_asset_split:Option<Vec<AAssetSplit>>,
	/// has many
	pub a_asset_transfer:Option<Vec<AAssetTransfer>>,
	/// has many
	pub a_depreciation_build:Option<Vec<ADepreciationBuild>>,
	/// has many
	pub a_depreciation_entry:Option<Vec<ADepreciationEntry>>,
	/// has many
	pub c_acctschema:Option<Vec<CAcctschema>>,
	/// has many
	pub c_periodcontrol:Option<Vec<CPeriodcontrol>>,
	/// has many
	pub fact_acct:Option<Vec<FactAcct>>,
	/// has many
	pub gl_journal:Option<Vec<GlJournal>>,
	/// has many
	pub gl_journalbatch:Option<Vec<GlJournalbatch>>,
	/// has many
	pub hr_period:Option<Vec<HrPeriod>>,
	/// has many
	pub i_fajournal:Option<Vec<IFajournal>>,
	/// has many
	pub i_gljournal:Option<Vec<IGljournal>>,
	/// has many
	pub m_demandline:Option<Vec<MDemandline>>,
	/// has many
	pub m_forecastline:Option<Vec<MForecastline>>,
}

pub struct CPeriodcontrol {
	/// primary
	/// not nullable 
	pub c_periodcontrol_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_period_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub docbasetype:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub periodaction:String,
	pub periodstatus:Option<String>,
	pub processing:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_period_id_c_period:Option<CPeriod>,
}

pub struct CPhase {
	/// primary
	/// not nullable 
	pub c_phase_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_projecttype_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub m_product_id:Option<f64>,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub seqno:f64,
	/// defaults to: 0
	/// not nullable 
	pub standardqty:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_projecttype_id_c_projecttype:Option<CProjecttype>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has many
	pub c_cyclephase:Option<Vec<CCyclephase>>,
	/// has many
	pub c_project:Option<Vec<CProject>>,
	/// has many
	pub c_projectphase:Option<Vec<CProjectphase>>,
	/// has many
	pub c_task:Option<Vec<CTask>>,
}

pub struct CPos {
	/// primary
	/// not nullable 
	pub c_pos_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub c_bankaccount_id:Option<f64>,
	pub c_bpartnercashtrx_id:Option<f64>,
	/// not nullable 
	pub c_cashbook_id:f64,
	pub c_doctype_id:Option<f64>,
	pub c_poskeylayout_id:Option<f64>,
	pub cashdrawer:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub ismodifyprice:String,
	/// not nullable 
	pub m_pricelist_id:f64,
	/// not nullable 
	pub m_warehouse_id:f64,
	/// not nullable 
	pub name:String,
	pub printername:Option<String>,
	/// not nullable 
	pub salesrep_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub salesrep_id_ad_user:Option<AdUser>,
	/// has one
	pub m_pricelist_id_m_pricelist:Option<MPricelist>,
	/// has one
	pub c_cashbook_id_c_cashbook:Option<CCashbook>,
	/// has one
	pub m_warehouse_id_m_warehouse:Option<MWarehouse>,
	/// has one
	pub c_poskeylayout_id_c_poskeylayout:Option<CPoskeylayout>,
	/// has one
	pub c_bpartnercashtrx_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub c_doctype_id_c_doctype:Option<CDoctype>,
	/// has one
	pub c_bankaccount_id_c_bankaccount:Option<CBankaccount>,
	/// has many
	pub c_order:Option<Vec<COrder>>,
}

pub struct CPoskey {
	/// primary
	/// not nullable 
	pub c_poskey_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_printcolor_id:Option<f64>,
	/// not nullable 
	pub c_poskeylayout_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub m_product_id:f64,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub qty:f64,
	/// defaults to: 0
	/// not nullable 
	pub seqno:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_poskeylayout_id_c_poskeylayout:Option<CPoskeylayout>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub ad_printcolor_id_ad_printcolor:Option<AdPrintcolor>,
}

pub struct CPoskeylayout {
	/// primary
	/// not nullable 
	pub c_poskeylayout_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has many
	pub c_pos:Option<Vec<CPos>>,
	/// has many
	pub c_poskey:Option<Vec<CPoskey>>,
}

pub struct CProject {
	/// primary
	/// not nullable 
	pub c_project_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_user_id:Option<f64>,
	pub c_bpartner_id:Option<f64>,
	pub c_bpartner_location_id:Option<f64>,
	pub c_bpartnersr_id:Option<f64>,
	pub c_campaign_id:Option<f64>,
	/// not nullable 
	pub c_currency_id:f64,
	pub c_paymentterm_id:Option<f64>,
	pub c_phase_id:Option<f64>,
	pub c_projecttype_id:Option<f64>,
	/// defaults to: 0
	/// not nullable 
	pub committedamt:f64,
	/// defaults to: 0
	/// not nullable 
	pub committedqty:f64,
	pub copyfrom:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub datecontract:Option<NaiveDateTime>,
	pub datefinish:Option<NaiveDateTime>,
	pub description:Option<String>,
	pub generateto:Option<String>,
	/// defaults to: 0
	/// not nullable 
	pub invoicedamt:f64,
	/// defaults to: 0
	/// not nullable 
	pub invoicedqty:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub iscommitceiling:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub iscommitment:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub issummary:String,
	pub m_pricelist_version_id:Option<f64>,
	pub m_warehouse_id:Option<f64>,
	/// not nullable 
	pub name:String,
	pub note:Option<String>,
	/// defaults to: 0
	/// not nullable 
	pub plannedamt:f64,
	/// defaults to: 0
	/// not nullable 
	pub plannedmarginamt:f64,
	/// defaults to: 0
	/// not nullable 
	pub plannedqty:f64,
	pub poreference:Option<String>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	pub processing:Option<String>,
	/// defaults to: 0
	/// not nullable 
	pub projectbalanceamt:f64,
	/// defaults to: 'N'::bpchar
	pub projectcategory:Option<String>,
	/// defaults to: 'P'::bpchar
	/// not nullable 
	pub projectlinelevel:String,
	/// defaults to: '-'::bpchar
	/// not nullable 
	pub projinvoicerule:String,
	pub salesrep_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
	/// has one
	pub ad_client_id_ad_client:Option<AdClient>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub c_bpartner_location_id_c_bpartner_location:Option<CBpartnerLocation>,
	/// has one
	pub c_paymentterm_id_c_paymentterm:Option<CPaymentterm>,
	/// has one
	pub c_currency_id_c_currency:Option<CCurrency>,
	/// has one
	pub m_pricelist_version_id_m_pricelist_version:Option<MPricelistVersion>,
	/// has one
	pub c_campaign_id_c_campaign:Option<CCampaign>,
	/// has one
	pub salesrep_id_ad_user:Option<AdUser>,
	/// has one
	pub c_projecttype_id_c_projecttype:Option<CProjecttype>,
	/// has one
	pub c_phase_id_c_phase:Option<CPhase>,
	/// has one
	pub m_warehouse_id_m_warehouse:Option<MWarehouse>,
	/// has one
	pub c_bpartnersr_id_c_bpartner:Option<CBpartner>,
	/// has many
	pub a_asset:Option<Vec<AAsset>>,
	/// has many
	pub c_acctschema_element:Option<Vec<CAcctschemaElement>>,
	/// has many
	pub c_cash:Option<Vec<CCash>>,
	/// has many
	pub c_invoice:Option<Vec<CInvoice>>,
	/// has many
	pub c_invoicebatchline:Option<Vec<CInvoicebatchline>>,
	/// has many
	pub c_invoiceline:Option<Vec<CInvoiceline>>,
	/// has many
	pub c_order:Option<Vec<COrder>>,
	/// has many
	pub c_orderline:Option<Vec<COrderline>>,
	/// has many
	pub c_payment:Option<Vec<CPayment>>,
	/// has many
	pub c_project_acct:Option<Vec<CProjectAcct>>,
	/// has many
	pub c_projectissue:Option<Vec<CProjectissue>>,
	/// has many
	pub c_projectline:Option<Vec<CProjectline>>,
	/// has many
	pub c_projectphase:Option<Vec<CProjectphase>>,
	/// has many
	pub c_recurring:Option<Vec<CRecurring>>,
	/// has many
	pub c_recurring_run:Option<Vec<CRecurringRun>>,
	/// has many
	pub c_validcombination:Option<Vec<CValidcombination>>,
	/// has many
	pub dd_order:Option<Vec<DdOrder>>,
	/// has many
	pub dd_orderline:Option<Vec<DdOrderline>>,
	/// has many
	pub fact_acct:Option<Vec<FactAcct>>,
	/// has many
	pub gl_distribution:Option<Vec<GlDistribution>>,
	/// has many
	pub gl_distributionline:Option<Vec<GlDistributionline>>,
	/// has many
	pub hr_contract:Option<Vec<HrContract>>,
	/// has many
	pub hr_movement:Option<Vec<HrMovement>>,
	/// has many
	pub i_fajournal:Option<Vec<IFajournal>>,
	/// has many
	pub i_gljournal:Option<Vec<IGljournal>>,
	/// has many
	pub i_invoice:Option<Vec<IInvoice>>,
	/// has many
	pub i_order:Option<Vec<IOrder>>,
	/// has many
	pub m_inout:Option<Vec<MInout>>,
	/// has many
	pub m_inoutline:Option<Vec<MInoutline>>,
	/// has many
	pub m_inventory:Option<Vec<MInventory>>,
	/// has many
	pub m_movement:Option<Vec<MMovement>>,
	/// has many
	pub m_production:Option<Vec<MProduction>>,
	/// has many
	pub pa_reportcolumn:Option<Vec<PaReportcolumn>>,
	/// has many
	pub pa_reportsource:Option<Vec<PaReportsource>>,
	/// has many
	pub pp_cost_collector:Option<Vec<PpCostCollector>>,
	/// has many
	pub pp_order:Option<Vec<PpOrder>>,
	/// has many
	pub r_issueproject:Option<Vec<RIssueproject>>,
	/// has many
	pub r_request:Option<Vec<RRequest>>,
	/// has many
	pub r_requestaction:Option<Vec<RRequestaction>>,
	/// has many
	pub s_timeexpenseline:Option<Vec<STimeexpenseline>>,
	/// has many
	pub t_aging:Option<Vec<TAging>>,
}

pub struct CProjectAcct {
	/// primary
	/// not nullable 
	pub c_project_id:f64,
	/// primary
	/// not nullable 
	pub c_acctschema_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub pj_asset_acct:f64,
	/// not nullable 
	pub pj_wip_acct:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_project_id_c_project:Option<CProject>,
	/// has one
	pub c_acctschema_id_c_acctschema:Option<CAcctschema>,
	/// has one
	pub pj_asset_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub pj_wip_acct_c_validcombination:Option<CValidcombination>,
}

pub struct CProjectissue {
	/// primary
	/// not nullable 
	pub c_projectissue_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_project_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub line:f64,
	/// defaults to: 0
	pub m_attributesetinstance_id:Option<f64>,
	pub m_inoutline_id:Option<f64>,
	/// not nullable 
	pub m_locator_id:f64,
	/// not nullable 
	pub m_product_id:f64,
	/// not nullable 
	pub movementdate:NaiveDateTime,
	/// defaults to: 0
	/// not nullable 
	pub movementqty:f64,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub posted:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	pub processing:Option<String>,
	pub s_timeexpenseline_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_project_id_c_project:Option<CProject>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub m_attributesetinstance_id_m_attributesetinstance:Option<MAttributesetinstance>,
	/// has one
	pub m_locator_id_m_locator:Option<MLocator>,
	/// has one
	pub s_timeexpenseline_id_s_timeexpenseline:Option<STimeexpenseline>,
	/// has one
	pub m_inoutline_id_m_inoutline:Option<MInoutline>,
	/// has many
	pub c_projectissuema:Option<Vec<CProjectissuema>>,
	/// has many
	pub c_projectline:Option<Vec<CProjectline>>,
	/// has many
	pub m_costdetail:Option<Vec<MCostdetail>>,
	/// has many
	pub m_transaction:Option<Vec<MTransaction>>,
	/// has many
	pub t_transaction:Option<Vec<TTransaction>>,
}

pub struct CProjectissuema {
	/// primary
	/// not nullable 
	pub c_projectissue_id:f64,
	/// primary
	/// not nullable 
	pub m_attributesetinstance_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 0
	/// not nullable 
	pub movementqty:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_projectissue_id_c_projectissue:Option<CProjectissue>,
	/// has one
	pub m_attributesetinstance_id_m_attributesetinstance:Option<MAttributesetinstance>,
}

pub struct CProjectline {
	/// primary
	/// not nullable 
	pub c_projectline_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub c_order_id:Option<f64>,
	pub c_orderpo_id:Option<f64>,
	/// not nullable 
	pub c_project_id:f64,
	pub c_projectissue_id:Option<f64>,
	pub c_projectphase_id:Option<f64>,
	pub c_projecttask_id:Option<f64>,
	/// defaults to: 0
	pub committedamt:Option<f64>,
	/// defaults to: 0
	pub committedqty:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	pub dopricing:Option<String>,
	/// defaults to: 0
	/// not nullable 
	pub invoicedamt:f64,
	/// defaults to: 0
	/// not nullable 
	pub invoicedqty:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isprinted:String,
	/// not nullable 
	pub line:f64,
	pub m_product_category_id:Option<f64>,
	pub m_product_id:Option<f64>,
	/// defaults to: 0
	/// not nullable 
	pub plannedamt:f64,
	/// defaults to: 0
	/// not nullable 
	pub plannedmarginamt:f64,
	/// defaults to: 0
	/// not nullable 
	pub plannedprice:f64,
	/// defaults to: 0
	/// not nullable 
	pub plannedqty:f64,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_project_id_c_project:Option<CProject>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub m_product_category_id_m_product_category:Option<MProductCategory>,
	/// has one
	pub c_projectissue_id_c_projectissue:Option<CProjectissue>,
	/// has one
	pub c_order_id_c_order:Option<COrder>,
	/// has one
	pub c_orderpo_id_c_order:Option<COrder>,
	/// has one
	pub c_projectphase_id_c_projectphase:Option<CProjectphase>,
	/// has one
	pub c_projecttask_id_c_projecttask:Option<CProjecttask>,
}

pub struct CProjectphase {
	/// primary
	/// not nullable 
	pub c_projectphase_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub c_order_id:Option<f64>,
	pub c_phase_id:Option<f64>,
	/// not nullable 
	pub c_project_id:f64,
	/// defaults to: 0
	/// not nullable 
	pub committedamt:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub enddate:Option<NaiveDateTime>,
	pub generateorder:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub iscommitceiling:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub iscomplete:String,
	pub m_product_id:Option<f64>,
	/// not nullable 
	pub name:String,
	/// defaults to: 0
	/// not nullable 
	pub plannedamt:f64,
	/// defaults to: 0
	pub priceactual:Option<f64>,
	pub projinvoicerule:Option<String>,
	/// defaults to: 0
	pub qty:Option<f64>,
	/// not nullable 
	pub seqno:f64,
	pub startdate:Option<NaiveDateTime>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_project_id_c_project:Option<CProject>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub c_order_id_c_order:Option<COrder>,
	/// has one
	pub c_phase_id_c_phase:Option<CPhase>,
	/// has many
	pub c_invoiceline:Option<Vec<CInvoiceline>>,
	/// has many
	pub c_orderline:Option<Vec<COrderline>>,
	/// has many
	pub c_projectline:Option<Vec<CProjectline>>,
	/// has many
	pub c_projecttask:Option<Vec<CProjecttask>>,
	/// has many
	pub fact_acct:Option<Vec<FactAcct>>,
	/// has many
	pub hr_movement:Option<Vec<HrMovement>>,
	/// has many
	pub m_inoutline:Option<Vec<MInoutline>>,
	/// has many
	pub s_timeexpenseline:Option<Vec<STimeexpenseline>>,
}

pub struct CProjecttask {
	/// primary
	/// not nullable 
	pub c_projecttask_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_projectphase_id:f64,
	pub c_task_id:Option<f64>,
	/// defaults to: 0
	/// not nullable 
	pub committedamt:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub m_product_id:Option<f64>,
	/// not nullable 
	pub name:String,
	/// defaults to: 0
	/// not nullable 
	pub plannedamt:f64,
	pub projinvoicerule:Option<String>,
	/// defaults to: 0
	pub qty:Option<f64>,
	/// not nullable 
	pub seqno:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_task_id_c_task:Option<CTask>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub c_projectphase_id_c_projectphase:Option<CProjectphase>,
	/// has many
	pub c_invoiceline:Option<Vec<CInvoiceline>>,
	/// has many
	pub c_orderline:Option<Vec<COrderline>>,
	/// has many
	pub c_projectline:Option<Vec<CProjectline>>,
	/// has many
	pub fact_acct:Option<Vec<FactAcct>>,
	/// has many
	pub hr_movement:Option<Vec<HrMovement>>,
	/// has many
	pub m_inoutline:Option<Vec<MInoutline>>,
	/// has many
	pub s_timeexpenseline:Option<Vec<STimeexpenseline>>,
}

pub struct CProjecttype {
	/// primary
	/// not nullable 
	pub c_projecttype_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub projectcategory:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has many
	pub c_phase:Option<Vec<CPhase>>,
	/// has many
	pub c_project:Option<Vec<CProject>>,
	/// has many
	pub pa_measure:Option<Vec<PaMeasure>>,
}

pub struct CRecurring {
	/// primary
	/// not nullable 
	pub c_recurring_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub c_invoice_id:Option<f64>,
	pub c_order_id:Option<f64>,
	pub c_payment_id:Option<f64>,
	pub c_project_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub datelastrun:Option<NaiveDateTime>,
	pub datenextrun:Option<NaiveDateTime>,
	pub description:Option<String>,
	pub frequency:Option<f64>,
	/// not nullable 
	pub frequencytype:String,
	pub gl_journalbatch_id:Option<f64>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	pub processing:Option<String>,
	/// not nullable 
	pub recurringtype:String,
	/// not nullable 
	pub runsmax:f64,
	/// not nullable 
	pub runsremaining:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_order_id_c_order:Option<COrder>,
	/// has one
	pub c_invoice_id_c_invoice:Option<CInvoice>,
	/// has one
	pub c_payment_id_c_payment:Option<CPayment>,
	/// has one
	pub c_project_id_c_project:Option<CProject>,
	/// has one
	pub gl_journalbatch_id_gl_journalbatch:Option<GlJournalbatch>,
	/// has many
	pub c_recurring_run:Option<Vec<CRecurringRun>>,
}

pub struct CRecurringRun {
	/// primary
	/// not nullable 
	pub c_recurring_run_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub c_invoice_id:Option<f64>,
	pub c_order_id:Option<f64>,
	pub c_payment_id:Option<f64>,
	pub c_project_id:Option<f64>,
	/// not nullable 
	pub c_recurring_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub datedoc:Option<NaiveDateTime>,
	pub gl_journalbatch_id:Option<f64>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_payment_id_c_payment:Option<CPayment>,
	/// has one
	pub c_order_id_c_order:Option<COrder>,
	/// has one
	pub c_invoice_id_c_invoice:Option<CInvoice>,
	/// has one
	pub c_project_id_c_project:Option<CProject>,
	/// has one
	pub gl_journalbatch_id_gl_journalbatch:Option<GlJournalbatch>,
	/// has one
	pub c_recurring_id_c_recurring:Option<CRecurring>,
}

pub struct CRegion {
	/// primary
	/// not nullable 
	pub c_region_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_country_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	pub isdefault:Option<String>,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_client_id_ad_client:Option<AdClient>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub c_country_id_c_country:Option<CCountry>,
	/// has many
	pub c_city:Option<Vec<CCity>>,
	/// has many
	pub c_location:Option<Vec<CLocation>>,
	/// has many
	pub c_tax:Option<Vec<CTax>>,
	/// has many
	pub i_bpartner:Option<Vec<IBpartner>>,
	/// has many
	pub i_invoice:Option<Vec<IInvoice>>,
	/// has many
	pub i_order:Option<Vec<IOrder>>,
	/// has many
	pub m_freight:Option<Vec<MFreight>>,
}

pub struct CRemuneration {
	/// primary
	/// not nullable 
	pub c_remuneration_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 0
	/// not nullable 
	pub grossramt:f64,
	/// defaults to: 0
	/// not nullable 
	pub grossrcost:f64,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// defaults to: 0
	/// not nullable 
	pub overtimeamt:f64,
	/// defaults to: 0
	/// not nullable 
	pub overtimecost:f64,
	/// not nullable 
	pub remunerationtype:String,
	/// defaults to: 0
	/// not nullable 
	pub standardhours:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has many
	pub c_jobremuneration:Option<Vec<CJobremuneration>>,
	/// has many
	pub c_userremuneration:Option<Vec<CUserremuneration>>,
}

pub struct CRevenuerecognition {
	/// primary
	/// not nullable 
	pub c_revenuerecognition_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub istimebased:String,
	/// not nullable 
	pub name:String,
	pub nomonths:Option<f64>,
	pub recognitionfrequency:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has many
	pub c_revenuerecognition_plan:Option<Vec<CRevenuerecognitionPlan>>,
	/// has many
	pub m_product:Option<Vec<MProduct>>,
}

pub struct CRevenuerecognitionPlan {
	/// primary
	/// not nullable 
	pub c_revenuerecognition_plan_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_acctschema_id:f64,
	/// not nullable 
	pub c_currency_id:f64,
	/// not nullable 
	pub c_invoiceline_id:f64,
	/// not nullable 
	pub c_revenuerecognition_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub p_revenue_acct:f64,
	/// defaults to: 0
	/// not nullable 
	pub recognizedamt:f64,
	/// defaults to: 0
	/// not nullable 
	pub totalamt:f64,
	/// not nullable 
	pub unearnedrevenue_acct:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_acctschema_id_c_acctschema:Option<CAcctschema>,
	/// has one
	pub c_revenuerecognition_id_c_revenuerecognition:Option<CRevenuerecognition>,
	/// has one
	pub c_invoiceline_id_c_invoiceline:Option<CInvoiceline>,
	/// has one
	pub unearnedrevenue_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub p_revenue_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub c_currency_id_c_currency:Option<CCurrency>,
	/// has many
	pub c_revenuerecognition_run:Option<Vec<CRevenuerecognitionRun>>,
	/// has many
	pub c_servicelevel:Option<Vec<CServicelevel>>,
}

pub struct CRevenuerecognitionRun {
	/// primary
	/// not nullable 
	pub c_revenuerecognition_run_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_revenuerecognition_plan_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub gl_journal_id:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 0
	/// not nullable 
	pub recognizedamt:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_revenuerecognition_plan_id_c_revenuerecognition_plan:Option<CRevenuerecognitionPlan>,
	/// has one
	pub gl_journal_id_gl_journal:Option<GlJournal>,
}

pub struct CRfq {
	/// primary
	/// not nullable 
	pub c_rfq_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_user_id:Option<f64>,
	pub c_bpartner_id:Option<f64>,
	pub c_bpartner_location_id:Option<f64>,
	/// not nullable 
	pub c_currency_id:f64,
	pub c_order_id:Option<f64>,
	/// not nullable 
	pub c_rfq_topic_id:f64,
	pub copylines:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub createpo:Option<String>,
	pub createso:Option<String>,
	/// not nullable 
	pub dateresponse:NaiveDateTime,
	pub dateworkcomplete:Option<NaiveDateTime>,
	pub dateworkstart:Option<NaiveDateTime>,
	/// defaults to: 0
	pub deliverydays:Option<f64>,
	pub description:Option<String>,
	/// defaults to: '.'::character varying
	/// not nullable 
	pub documentno:String,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isinvitedvendorsonly:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isquoteallqty:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isquotetotalamt:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isrfqresponseaccepted:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isselfservice:String,
	pub margin:Option<f64>,
	/// not nullable 
	pub name:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	pub processing:Option<String>,
	pub publishrfq:Option<String>,
	/// not nullable 
	pub quotetype:String,
	pub rankrfq:Option<String>,
	/// not nullable 
	pub salesrep_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_rfq_topic_id_c_rfq_topic:Option<CRfqTopic>,
	/// has one
	pub c_currency_id_c_currency:Option<CCurrency>,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub c_bpartner_location_id_c_bpartner_location:Option<CBpartnerLocation>,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
	/// has one
	pub salesrep_id_ad_user:Option<AdUser>,
	/// has one
	pub c_order_id_c_order:Option<COrder>,
	/// has many
	pub c_rfqline:Option<Vec<CRfqline>>,
	/// has many
	pub c_rfqresponse:Option<Vec<CRfqresponse>>,
}

pub struct CRfqTopic {
	/// primary
	/// not nullable 
	pub c_rfq_topic_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_printformat_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isselfservice:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_printformat_id_ad_printformat:Option<AdPrintformat>,
	/// has many
	pub c_rfq:Option<Vec<CRfq>>,
	/// has many
	pub c_rfq_topicsubscriber:Option<Vec<CRfqTopicsubscriber>>,
}

pub struct CRfqTopicsubscriber {
	/// primary
	/// not nullable 
	pub c_rfq_topicsubscriber_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_user_id:Option<f64>,
	/// not nullable 
	pub c_bpartner_id:f64,
	/// not nullable 
	pub c_bpartner_location_id:f64,
	/// not nullable 
	pub c_rfq_topic_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub optoutdate:Option<NaiveDateTime>,
	pub subscribedate:Option<NaiveDateTime>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_rfq_topic_id_c_rfq_topic:Option<CRfqTopic>,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub c_bpartner_location_id_c_bpartner_location:Option<CBpartnerLocation>,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
	/// has many
	pub c_rfq_topicsubscriberonly:Option<Vec<CRfqTopicsubscriberonly>>,
}

pub struct CRfqTopicsubscriberonly {
	/// primary
	/// not nullable 
	pub c_rfq_topicsubscriberonly_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_rfq_topicsubscriber_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub m_product_category_id:Option<f64>,
	pub m_product_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_rfq_topicsubscriber_id_c_rfq_topicsubscriber:Option<CRfqTopicsubscriber>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub m_product_category_id_m_product_category:Option<MProductCategory>,
}

pub struct CRfqline {
	/// primary
	/// not nullable 
	pub c_rfqline_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_rfq_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub dateworkcomplete:Option<NaiveDateTime>,
	pub dateworkstart:Option<NaiveDateTime>,
	/// defaults to: 0
	pub deliverydays:Option<f64>,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 0
	/// not nullable 
	pub line:f64,
	/// defaults to: 0
	pub m_attributesetinstance_id:Option<f64>,
	pub m_product_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_rfq_id_c_rfq:Option<CRfq>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub m_attributesetinstance_id_m_attributesetinstance:Option<MAttributesetinstance>,
	/// has many
	pub c_rfqlineqty:Option<Vec<CRfqlineqty>>,
	/// has many
	pub c_rfqresponseline:Option<Vec<CRfqresponseline>>,
}

pub struct CRfqlineqty {
	/// primary
	/// not nullable 
	pub c_rfqlineqty_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: 0
	/// not nullable 
	pub benchmarkprice:f64,
	/// defaults to: 0
	pub bestresponseamt:Option<f64>,
	/// not nullable 
	pub c_rfqline_id:f64,
	/// not nullable 
	pub c_uom_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isofferqty:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub ispurchaseqty:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isrfqqty:String,
	pub margin:Option<f64>,
	/// defaults to: 0
	pub offeramt:Option<f64>,
	/// defaults to: 0
	/// not nullable 
	pub qty:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_rfqline_id_c_rfqline:Option<CRfqline>,
	/// has one
	pub c_uom_id_c_uom:Option<CUom>,
	/// has many
	pub c_rfqresponselineqty:Option<Vec<CRfqresponselineqty>>,
}

pub struct CRfqresponse {
	/// primary
	/// not nullable 
	pub c_rfqresponse_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_user_id:Option<f64>,
	/// not nullable 
	pub c_bpartner_id:f64,
	/// not nullable 
	pub c_bpartner_location_id:f64,
	/// not nullable 
	pub c_currency_id:f64,
	pub c_order_id:Option<f64>,
	/// not nullable 
	pub c_rfq_id:f64,
	pub checkcomplete:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub dateinvited:Option<NaiveDateTime>,
	pub dateresponse:Option<NaiveDateTime>,
	pub dateworkcomplete:Option<NaiveDateTime>,
	pub dateworkstart:Option<NaiveDateTime>,
	/// defaults to: 0
	pub deliverydays:Option<f64>,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub iscomplete:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isselectedwinner:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isselfservice:String,
	/// not nullable 
	pub name:String,
	/// defaults to: 0
	/// not nullable 
	pub price:f64,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	pub processing:Option<String>,
	/// defaults to: 0
	pub ranking:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_rfq_id_c_rfq:Option<CRfq>,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub c_bpartner_location_id_c_bpartner_location:Option<CBpartnerLocation>,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
	/// has one
	pub c_currency_id_c_currency:Option<CCurrency>,
	/// has one
	pub c_order_id_c_order:Option<COrder>,
	/// has many
	pub c_rfqresponseline:Option<Vec<CRfqresponseline>>,
}

pub struct CRfqresponseline {
	/// primary
	/// not nullable 
	pub c_rfqresponseline_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_rfqline_id:f64,
	/// not nullable 
	pub c_rfqresponse_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub dateworkcomplete:Option<NaiveDateTime>,
	pub dateworkstart:Option<NaiveDateTime>,
	/// defaults to: 0
	pub deliverydays:Option<f64>,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isselectedwinner:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isselfservice:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_rfqline_id_c_rfqline:Option<CRfqline>,
	/// has one
	pub c_rfqresponse_id_c_rfqresponse:Option<CRfqresponse>,
	/// has many
	pub c_rfqresponselineqty:Option<Vec<CRfqresponselineqty>>,
}

pub struct CRfqresponselineqty {
	/// primary
	/// not nullable 
	pub c_rfqresponselineqty_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_rfqlineqty_id:f64,
	/// not nullable 
	pub c_rfqresponseline_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub discount:Option<f64>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 0
	/// not nullable 
	pub price:f64,
	/// defaults to: 0
	pub ranking:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_rfqresponseline_id_c_rfqresponseline:Option<CRfqresponseline>,
	/// has one
	pub c_rfqlineqty_id_c_rfqlineqty:Option<CRfqlineqty>,
}

pub struct CSalesregion {
	/// primary
	/// not nullable 
	pub c_salesregion_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isdefault:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub issummary:String,
	/// not nullable 
	pub name:String,
	pub salesrep_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
	/// has one
	pub salesrep_id_ad_user:Option<AdUser>,
	/// has many
	pub c_acctschema_element:Option<Vec<CAcctschemaElement>>,
	/// has many
	pub c_bpartner_location:Option<Vec<CBpartnerLocation>>,
	/// has many
	pub c_commissionline:Option<Vec<CCommissionline>>,
	/// has many
	pub c_validcombination:Option<Vec<CValidcombination>>,
	/// has many
	pub fact_acct:Option<Vec<FactAcct>>,
	/// has many
	pub gl_distribution:Option<Vec<GlDistribution>>,
	/// has many
	pub gl_distributionline:Option<Vec<GlDistributionline>>,
	/// has many
	pub i_fajournal:Option<Vec<IFajournal>>,
	/// has many
	pub i_gljournal:Option<Vec<IGljournal>>,
	/// has many
	pub pa_reportcolumn:Option<Vec<PaReportcolumn>>,
	/// has many
	pub pa_reportsource:Option<Vec<PaReportsource>>,
}

pub struct CServicelevel {
	/// primary
	/// not nullable 
	pub c_servicelevel_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_revenuerecognition_plan_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub m_product_id:f64,
	/// defaults to: 'N'::bpchar
	pub processed:Option<String>,
	pub processing:Option<String>,
	/// not nullable 
	pub servicelevelinvoiced:f64,
	/// not nullable 
	pub servicelevelprovided:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub c_revenuerecognition_plan_id_c_revenuerecognition_plan:Option<CRevenuerecognitionPlan>,
	/// has many
	pub c_servicelevelline:Option<Vec<CServicelevelline>>,
}

pub struct CServicelevelline {
	/// primary
	/// not nullable 
	pub c_servicelevelline_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_servicelevel_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	pub processed:Option<String>,
	/// not nullable 
	pub servicedate:NaiveDateTime,
	/// not nullable 
	pub servicelevelprovided:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_servicelevel_id_c_servicelevel:Option<CServicelevel>,
}

pub struct CSubacct {
	/// primary
	/// not nullable 
	pub c_subacct_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_elementvalue_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
	/// has one
	pub c_elementvalue_id_c_elementvalue:Option<CElementvalue>,
	/// has many
	pub c_validcombination:Option<Vec<CValidcombination>>,
	/// has many
	pub fact_acct:Option<Vec<FactAcct>>,
}

pub struct CSubscription {
	/// primary
	/// not nullable 
	pub c_subscription_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_bpartner_id:f64,
	/// not nullable 
	pub c_subscriptiontype_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isdue:String,
	/// not nullable 
	pub m_product_id:f64,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub paiduntildate:NaiveDateTime,
	/// not nullable 
	pub renewaldate:NaiveDateTime,
	/// not nullable 
	pub startdate:NaiveDateTime,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub c_subscriptiontype_id_c_subscriptiontype:Option<CSubscriptiontype>,
	/// has many
	pub c_subscription_delivery:Option<Vec<CSubscriptionDelivery>>,
}

pub struct CSubscriptionDelivery {
	/// primary
	/// not nullable 
	pub c_subscription_delivery_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_subscription_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_subscription_id_c_subscription:Option<CSubscription>,
}

pub struct CSubscriptiontype {
	/// primary
	/// not nullable 
	pub c_subscriptiontype_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	pub ad_org_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// not nullable 
	pub frequency:f64,
	/// not nullable 
	pub frequencytype:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has many
	pub c_subscription:Option<Vec<CSubscription>>,
	/// has many
	pub m_product:Option<Vec<MProduct>>,
}

pub struct CTask {
	/// primary
	/// not nullable 
	pub c_task_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_phase_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub m_product_id:Option<f64>,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub seqno:f64,
	/// defaults to: 0
	/// not nullable 
	pub standardqty:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_phase_id_c_phase:Option<CPhase>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has many
	pub c_projecttask:Option<Vec<CProjecttask>>,
}

pub struct CTax {
	/// primary
	/// not nullable 
	pub c_tax_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_rule_id:Option<f64>,
	pub c_country_id:Option<f64>,
	pub c_region_id:Option<f64>,
	/// not nullable 
	pub c_taxcategory_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isdefault:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isdocumentlevel:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub issalestax:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub issummary:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub istaxexempt:String,
	/// not nullable 
	pub name:String,
	pub parent_tax_id:Option<f64>,
	/// not nullable 
	pub rate:f64,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub requirestaxcertificate:String,
	/// defaults to: 'B'::bpchar
	/// not nullable 
	pub sopotype:String,
	pub taxindicator:Option<String>,
	pub to_country_id:Option<f64>,
	pub to_region_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub validfrom:NaiveDateTime,
	/// has one, self referential
	pub parent_tax_id_c_tax:Option<Box<CTax>>,
	/// has one
	pub c_country_id_c_country:Option<CCountry>,
	/// has one
	pub c_region_id_c_region:Option<CRegion>,
	/// has one
	pub to_country_id_c_country:Option<CCountry>,
	/// has one
	pub to_region_id_c_region:Option<CRegion>,
	/// has one
	pub c_taxcategory_id_c_taxcategory:Option<CTaxcategory>,
	/// has one
	pub ad_rule_id_ad_rule:Option<AdRule>,
	/// has many
	pub c_invoicebatchline:Option<Vec<CInvoicebatchline>>,
	/// has many
	pub c_invoiceline:Option<Vec<CInvoiceline>>,
	/// has many
	pub c_invoicetax:Option<Vec<CInvoicetax>>,
	/// has many
	pub c_orderline:Option<Vec<COrderline>>,
	/// has many
	pub c_ordertax:Option<Vec<COrdertax>>,
	/// has many
	pub c_tax:Option<Vec<CTax>>,
	/// has many
	pub c_tax_acct:Option<Vec<CTaxAcct>>,
	/// has many
	pub c_tax_trl:Option<Vec<CTaxTrl>>,
	/// has many
	pub c_taxdeclarationline:Option<Vec<CTaxdeclarationline>>,
	/// has many
	pub c_taxdefinition:Option<Vec<CTaxdefinition>>,
	/// has many
	pub c_taxpostal:Option<Vec<CTaxpostal>>,
	/// has many
	pub fact_acct:Option<Vec<FactAcct>>,
	/// has many
	pub i_invoice:Option<Vec<IInvoice>>,
	/// has many
	pub i_order:Option<Vec<IOrder>>,
}

pub struct CTaxAcct {
	/// primary
	/// not nullable 
	pub c_tax_id:f64,
	/// primary
	/// not nullable 
	pub c_acctschema_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub t_credit_acct:f64,
	/// not nullable 
	pub t_due_acct:f64,
	/// not nullable 
	pub t_expense_acct:f64,
	/// not nullable 
	pub t_liability_acct:f64,
	/// not nullable 
	pub t_receivables_acct:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_tax_id_c_tax:Option<CTax>,
	/// has one
	pub c_acctschema_id_c_acctschema:Option<CAcctschema>,
	/// has one
	pub t_due_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub t_liability_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub t_credit_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub t_receivables_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub t_expense_acct_c_validcombination:Option<CValidcombination>,
}

pub struct CTaxTrl {
	/// primary
	/// not nullable 
	pub c_tax_id:f64,
	/// primary
	/// not nullable 
	pub ad_language:String,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub istranslated:String,
	/// not nullable 
	pub name:String,
	pub taxindicator:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_tax_id_c_tax:Option<CTax>,
	/// has one
	pub ad_language_ad_language:Option<AdLanguage>,
}

pub struct CTaxbase {
	/// primary
	/// not nullable 
	pub c_taxbase_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub base:Option<String>,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	pub percentage:Option<f64>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
	/// has many
	pub c_taxdefinition:Option<Vec<CTaxdefinition>>,
}

pub struct CTaxcategory {
	/// primary
	/// not nullable 
	pub c_taxcategory_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub commoditycode:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isdefault:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has many
	pub c_charge:Option<Vec<CCharge>>,
	/// has many
	pub c_tax:Option<Vec<CTax>>,
	/// has many
	pub c_taxcategory_trl:Option<Vec<CTaxcategoryTrl>>,
	/// has many
	pub c_taxdefinition:Option<Vec<CTaxdefinition>>,
	/// has many
	pub m_product:Option<Vec<MProduct>>,
	/// has many
	pub s_expensetype:Option<Vec<SExpensetype>>,
	/// has many
	pub s_resourcetype:Option<Vec<SResourcetype>>,
	/// has many
	pub s_training:Option<Vec<STraining>>,
}

pub struct CTaxcategoryTrl {
	/// primary
	/// not nullable 
	pub c_taxcategory_id:f64,
	/// primary
	/// not nullable 
	pub ad_language:String,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub istranslated:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_taxcategory_id_c_taxcategory:Option<CTaxcategory>,
	/// has one
	pub ad_language_ad_language:Option<AdLanguage>,
}

pub struct CTaxdeclaration {
	/// primary
	/// not nullable 
	pub c_taxdeclaration_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub datefrom:NaiveDateTime,
	/// not nullable 
	pub dateto:NaiveDateTime,
	/// not nullable 
	pub datetrx:NaiveDateTime,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	pub processing:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has many
	pub c_taxdeclarationacct:Option<Vec<CTaxdeclarationacct>>,
	/// has many
	pub c_taxdeclarationline:Option<Vec<CTaxdeclarationline>>,
}

pub struct CTaxdeclarationacct {
	/// primary
	/// not nullable 
	pub c_taxdeclarationacct_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_acctschema_id:f64,
	/// not nullable 
	pub c_taxdeclaration_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// not nullable 
	pub fact_acct_id:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub line:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_taxdeclaration_id_c_taxdeclaration:Option<CTaxdeclaration>,
	/// has one
	pub fact_acct_id_fact_acct:Option<FactAcct>,
	/// has one
	pub c_acctschema_id_c_acctschema:Option<CAcctschema>,
}

pub struct CTaxdeclarationline {
	/// primary
	/// not nullable 
	pub c_taxdeclarationline_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub c_allocationline_id:Option<f64>,
	/// not nullable 
	pub c_bpartner_id:f64,
	/// not nullable 
	pub c_currency_id:f64,
	pub c_invoice_id:Option<f64>,
	pub c_invoiceline_id:Option<f64>,
	/// not nullable 
	pub c_tax_id:f64,
	/// not nullable 
	pub c_taxdeclaration_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub dateacct:NaiveDateTime,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub ismanual:String,
	/// defaults to: 0
	/// not nullable 
	pub line:f64,
	/// defaults to: 0
	/// not nullable 
	pub taxamt:f64,
	/// defaults to: 0
	/// not nullable 
	pub taxbaseamt:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_taxdeclaration_id_c_taxdeclaration:Option<CTaxdeclaration>,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub c_tax_id_c_tax:Option<CTax>,
	/// has one
	pub c_invoice_id_c_invoice:Option<CInvoice>,
	/// has one
	pub c_invoiceline_id_c_invoiceline:Option<CInvoiceline>,
	/// has one
	pub c_allocationline_id_c_allocationline:Option<CAllocationline>,
	/// has one
	pub c_currency_id_c_currency:Option<CCurrency>,
}

pub struct CTaxdefinition {
	/// primary
	/// not nullable 
	pub c_taxdefinition_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_orgtype_id:Option<f64>,
	pub c_bp_group_id:Option<f64>,
	pub c_bpartner_id:Option<f64>,
	pub c_tax_id:Option<f64>,
	pub c_taxbase_id:Option<f64>,
	pub c_taxcategory_id:Option<f64>,
	pub c_taxgroup_id:Option<f64>,
	pub c_taxtype_id:Option<f64>,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// not nullable 
	pub isactive:String,
	pub isinvoiced:Option<String>,
	pub m_product_category_id:Option<f64>,
	pub m_product_id:Option<f64>,
	pub maxtaxable:Option<f64>,
	pub mintaxable:Option<f64>,
	/// not nullable 
	pub name:String,
	pub seqno:Option<f64>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub validfrom:Option<NaiveDateTime>,
	pub validto:Option<NaiveDateTime>,
	/// not nullable 
	pub value:String,
	/// has one
	pub c_bp_group_id_c_bp_group:Option<CBpGroup>,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub c_taxbase_id_c_taxbase:Option<CTaxbase>,
	/// has one
	pub c_taxcategory_id_c_taxcategory:Option<CTaxcategory>,
	/// has one
	pub c_taxgroup_id_c_taxgroup:Option<CTaxgroup>,
	/// has one
	pub c_taxtype_id_c_taxtype:Option<CTaxtype>,
	/// has one
	pub c_tax_id_c_tax:Option<CTax>,
	/// has one
	pub m_product_category_id_m_product_category:Option<MProductCategory>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub ad_orgtype_id_ad_orgtype:Option<AdOrgtype>,
}

pub struct CTaxgroup {
	/// primary
	/// not nullable 
	pub c_taxgroup_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
	/// has many
	pub c_bpartner:Option<Vec<CBpartner>>,
	/// has many
	pub c_taxdefinition:Option<Vec<CTaxdefinition>>,
}

pub struct CTaxpostal {
	/// primary
	/// not nullable 
	pub c_taxpostal_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_tax_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub postal:String,
	pub postal_to:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_tax_id_c_tax:Option<CTax>,
}

pub struct CTaxtype {
	/// primary
	/// not nullable 
	pub c_taxtype_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
	/// has many
	pub c_taxdefinition:Option<Vec<CTaxdefinition>>,
}

pub struct CUom {
	/// primary
	/// not nullable 
	pub c_uom_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub costingprecision:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isdefault:String,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub stdprecision:f64,
	pub uomsymbol:Option<String>,
	/// defaults to: NULL::character varying
	pub uomtype:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub x12de355:String,
	/// has one
	pub ad_client_id_ad_client:Option<AdClient>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has many
	pub ad_clientinfo:Option<Vec<AdClientinfo>>,
	/// has many
	pub c_invoiceline:Option<Vec<CInvoiceline>>,
	/// has many
	pub c_orderline:Option<Vec<COrderline>>,
	/// has many
	pub c_rfqlineqty:Option<Vec<CRfqlineqty>>,
	/// has many
	pub c_uom_conversion:Option<Vec<CUomConversion>>,
	/// has many
	pub c_uom_trl:Option<Vec<CUomTrl>>,
	/// has many
	pub dd_orderline:Option<Vec<DdOrderline>>,
	/// has many
	pub fact_acct:Option<Vec<FactAcct>>,
	/// has many
	pub gl_journalline:Option<Vec<GlJournalline>>,
	/// has many
	pub i_fajournal:Option<Vec<IFajournal>>,
	/// has many
	pub i_gljournal:Option<Vec<IGljournal>>,
	/// has many
	pub i_order:Option<Vec<IOrder>>,
	/// has many
	pub i_product:Option<Vec<IProduct>>,
	/// has many
	pub m_inoutline:Option<Vec<MInoutline>>,
	/// has many
	pub m_product:Option<Vec<MProduct>>,
	/// has many
	pub m_product_po:Option<Vec<MProductPo>>,
	/// has many
	pub pp_cost_collector:Option<Vec<PpCostCollector>>,
	/// has many
	pub pp_order:Option<Vec<PpOrder>>,
	/// has many
	pub pp_order_bom:Option<Vec<PpOrderBom>>,
	/// has many
	pub pp_order_bomline:Option<Vec<PpOrderBomline>>,
	/// has many
	pub pp_product_bom:Option<Vec<PpProductBom>>,
	/// has many
	pub pp_product_bomline:Option<Vec<PpProductBomline>>,
	/// has many
	pub s_expensetype:Option<Vec<SExpensetype>>,
	/// has many
	pub s_resourcetype:Option<Vec<SResourcetype>>,
	/// has many
	pub s_timeexpenseline:Option<Vec<STimeexpenseline>>,
	/// has many
	pub s_training:Option<Vec<STraining>>,
}

pub struct CUomConversion {
	/// primary
	/// not nullable 
	pub c_uom_conversion_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_uom_id:f64,
	/// not nullable 
	pub c_uom_to_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 0
	/// not nullable 
	pub dividerate:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub m_product_id:Option<f64>,
	/// defaults to: 0
	/// not nullable 
	pub multiplyrate:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_client_id_ad_client:Option<AdClient>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub c_uom_id_c_uom:Option<CUom>,
	/// has one
	pub c_uom_to_id_c_uom:Option<CUom>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
}

pub struct CUomTrl {
	/// primary
	/// not nullable 
	pub c_uom_id:f64,
	/// primary
	/// not nullable 
	pub ad_language:String,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub istranslated:String,
	/// not nullable 
	pub name:String,
	pub uomsymbol:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_uom_id_c_uom:Option<CUom>,
	/// has one
	pub ad_language_ad_language:Option<AdLanguage>,
}

pub struct CUserremuneration {
	/// primary
	/// not nullable 
	pub c_userremuneration_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_user_id:f64,
	/// not nullable 
	pub c_remuneration_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 0
	/// not nullable 
	pub grossramt:f64,
	/// defaults to: 0
	/// not nullable 
	pub grossrcost:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 0
	/// not nullable 
	pub overtimeamt:f64,
	/// defaults to: 0
	/// not nullable 
	pub overtimecost:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub validfrom:NaiveDateTime,
	pub validto:Option<NaiveDateTime>,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
	/// has one
	pub c_remuneration_id_c_remuneration:Option<CRemuneration>,
}

pub struct CValidcombination {
	/// primary
	/// not nullable 
	pub c_validcombination_id:f64,
	/// not nullable 
	pub account_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_orgtrx_id:Option<f64>,
	pub alias:Option<String>,
	/// not nullable 
	pub c_acctschema_id:f64,
	pub c_activity_id:Option<f64>,
	pub c_bpartner_id:Option<f64>,
	pub c_campaign_id:Option<f64>,
	pub c_locfrom_id:Option<f64>,
	pub c_locto_id:Option<f64>,
	pub c_project_id:Option<f64>,
	pub c_salesregion_id:Option<f64>,
	pub c_subacct_id:Option<f64>,
	pub combination:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isfullyqualified:String,
	pub m_product_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub user1_id:Option<f64>,
	pub user2_id:Option<f64>,
	pub userelement1_id:Option<f64>,
	pub userelement2_id:Option<f64>,
	/// has one
	pub ad_client_id_ad_client:Option<AdClient>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub c_acctschema_id_c_acctschema:Option<CAcctschema>,
	/// has one
	pub account_id_c_elementvalue:Option<CElementvalue>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub ad_orgtrx_id_ad_org:Option<AdOrg>,
	/// has one
	pub c_locfrom_id_c_location:Option<CLocation>,
	/// has one
	pub c_locto_id_c_location:Option<CLocation>,
	/// has one
	pub c_salesregion_id_c_salesregion:Option<CSalesregion>,
	/// has one
	pub c_project_id_c_project:Option<CProject>,
	/// has one
	pub c_campaign_id_c_campaign:Option<CCampaign>,
	/// has one
	pub c_activity_id_c_activity:Option<CActivity>,
	/// has one
	pub user1_id_c_elementvalue:Option<CElementvalue>,
	/// has one
	pub user2_id_c_elementvalue:Option<CElementvalue>,
	/// has one
	pub c_subacct_id_c_subacct:Option<CSubacct>,
	/// has many
	pub c_acctschema_default:Option<Vec<CAcctschemaDefault>>,
	/// has many
	pub c_acctschema_gl:Option<Vec<CAcctschemaGl>>,
	/// has many
	pub c_bankaccount_acct:Option<Vec<CBankaccountAcct>>,
	/// has many
	pub c_bp_customer_acct:Option<Vec<CBpCustomerAcct>>,
	/// has many
	pub c_bp_employee_acct:Option<Vec<CBpEmployeeAcct>>,
	/// has many
	pub c_bp_group_acct:Option<Vec<CBpGroupAcct>>,
	/// has many
	pub c_bp_vendor_acct:Option<Vec<CBpVendorAcct>>,
	/// has many
	pub c_cashbook_acct:Option<Vec<CCashbookAcct>>,
	/// has many
	pub c_charge_acct:Option<Vec<CChargeAcct>>,
	/// has many
	pub c_currency_acct:Option<Vec<CCurrencyAcct>>,
	/// has many
	pub c_interorg_acct:Option<Vec<CInterorgAcct>>,
	/// has many
	pub c_project_acct:Option<Vec<CProjectAcct>>,
	/// has many
	pub c_revenuerecognition_plan:Option<Vec<CRevenuerecognitionPlan>>,
	/// has many
	pub c_tax_acct:Option<Vec<CTaxAcct>>,
	/// has many
	pub c_withholding_acct:Option<Vec<CWithholdingAcct>>,
	/// has many
	pub gl_journalline:Option<Vec<GlJournalline>>,
	/// has many
	pub i_fajournal:Option<Vec<IFajournal>>,
	/// has many
	pub i_gljournal:Option<Vec<IGljournal>>,
	/// has many
	pub m_product_acct:Option<Vec<MProductAcct>>,
	/// has many
	pub m_product_category_acct:Option<Vec<MProductCategoryAcct>>,
	/// has many
	pub m_warehouse_acct:Option<Vec<MWarehouseAcct>>,
	/// has many
	pub test:Option<Vec<Test>>,
}

pub struct CWithholding {
	/// primary
	/// not nullable 
	pub c_withholding_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub beneficiary:Option<f64>,
	/// not nullable 
	pub c_paymentterm_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 0
	pub fixamt:Option<f64>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub ispaidto3party:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub ispercentwithholding:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub istaxprorated:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub istaxwithholding:String,
	/// defaults to: 0
	pub maxamt:Option<f64>,
	/// defaults to: 0
	pub minamt:Option<f64>,
	/// not nullable 
	pub name:String,
	/// defaults to: 0
	pub percent:Option<f64>,
	/// defaults to: 0
	pub thresholdmax:Option<f64>,
	/// defaults to: 0
	pub thresholdmin:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_paymentterm_id_c_paymentterm:Option<CPaymentterm>,
	/// has one
	pub beneficiary_c_bpartner:Option<CBpartner>,
	/// has many
	pub c_bp_withholding:Option<Vec<CBpWithholding>>,
	/// has many
	pub c_withholding_acct:Option<Vec<CWithholdingAcct>>,
}

pub struct CWithholdingAcct {
	/// primary
	/// not nullable 
	pub c_withholding_id:f64,
	/// primary
	/// not nullable 
	pub c_acctschema_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub withholding_acct:f64,
	/// has one
	pub c_withholding_id_c_withholding:Option<CWithholding>,
	/// has one
	pub c_acctschema_id_c_acctschema:Option<CAcctschema>,
	/// has one
	pub withholding_acct_c_validcombination:Option<CValidcombination>,
}

pub struct CYear {
	/// primary
	/// not nullable 
	pub c_year_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_calendar_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// not nullable 
	pub fiscalyear:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub processing:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_client_id_ad_client:Option<AdClient>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub c_calendar_id_c_calendar:Option<CCalendar>,
	/// has many
	pub c_period:Option<Vec<CPeriod>>,
	/// has many
	pub hr_period:Option<Vec<HrPeriod>>,
	/// has many
	pub hr_year:Option<Vec<HrYear>>,
	/// has many
	pub m_demand:Option<Vec<MDemand>>,
	/// has many
	pub m_forecast:Option<Vec<MForecast>>,
}

pub struct Classpackage {
	/// primary
	/// not nullable 
	pub classpackage_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: NULL::character varying
	pub description:Option<String>,
	/// defaults to: NULL::character varying
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
}

pub struct CmAccesscontainer {
	/// primary
	/// not nullable 
	pub cm_accessprofile_id:f64,
	/// primary
	/// not nullable 
	pub cm_container_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub cm_accessprofile_id_cm_accessprofile:Option<CmAccessprofile>,
	/// has one
	pub cm_container_id_cm_container:Option<CmContainer>,
}

pub struct CmAccesslistbpgroup {
	/// primary
	/// not nullable 
	pub cm_accessprofile_id:f64,
	/// primary
	/// not nullable 
	pub c_bp_group_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub cm_accessprofile_id_cm_accessprofile:Option<CmAccessprofile>,
	/// has one
	pub c_bp_group_id_c_bp_group:Option<CBpGroup>,
}

pub struct CmAccesslistrole {
	/// primary
	/// not nullable 
	pub cm_accessprofile_id:f64,
	/// primary
	/// not nullable 
	pub ad_role_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub cm_accessprofile_id_cm_accessprofile:Option<CmAccessprofile>,
	/// has one
	pub ad_role_id_ad_role:Option<AdRole>,
}

pub struct CmAccessmedia {
	/// primary
	/// not nullable 
	pub cm_media_id:f64,
	/// primary
	/// not nullable 
	pub cm_accessprofile_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub cm_media_id_cm_media:Option<CmMedia>,
	/// has one
	pub cm_accessprofile_id_cm_accessprofile:Option<CmAccessprofile>,
}

pub struct CmAccessnewschannel {
	/// primary
	/// not nullable 
	pub cm_accessprofile_id:f64,
	/// primary
	/// not nullable 
	pub cm_newschannel_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub cm_accessprofile_id_cm_accessprofile:Option<CmAccessprofile>,
	/// has one
	pub cm_newschannel_id_cm_newschannel:Option<CmNewschannel>,
}

pub struct CmAccessprofile {
	/// primary
	/// not nullable 
	pub cm_accessprofile_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isexclude:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has many
	pub cm_accesscontainer:Option<Vec<CmAccesscontainer>>,
	/// has many
	pub cm_accesslistbpgroup:Option<Vec<CmAccesslistbpgroup>>,
	/// has many
	pub cm_accesslistrole:Option<Vec<CmAccesslistrole>>,
	/// has many
	pub cm_accessmedia:Option<Vec<CmAccessmedia>>,
	/// has many
	pub cm_accessnewschannel:Option<Vec<CmAccessnewschannel>>,
	/// has many
	pub cm_accessstage:Option<Vec<CmAccessstage>>,
}

pub struct CmAccessstage {
	/// primary
	/// not nullable 
	pub cm_accessprofile_id:f64,
	/// primary
	/// not nullable 
	pub cm_cstage_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub cm_accessprofile_id_cm_accessprofile:Option<CmAccessprofile>,
	/// has one
	pub cm_cstage_id_cm_cstage:Option<CmCstage>,
}

pub struct CmAd {
	/// primary
	/// not nullable 
	pub cm_ad_id:f64,
	/// defaults to: 0
	/// not nullable 
	pub actualclick:f64,
	/// defaults to: 0
	/// not nullable 
	pub actualimpression:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub cm_ad_cat_id:f64,
	/// not nullable 
	pub cm_media_id:f64,
	pub contenthtml:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub enddate:Option<NaiveDateTime>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isadflag:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub islogged:String,
	/// defaults to: 0
	/// not nullable 
	pub maxclick:f64,
	/// defaults to: 0
	/// not nullable 
	pub maximpression:f64,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub startdate:NaiveDateTime,
	/// defaults to: 0
	/// not nullable 
	pub startimpression:f64,
	/// not nullable 
	pub target_frame:String,
	pub targeturl:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub cm_ad_cat_id_cm_ad_cat:Option<CmAdCat>,
	/// has one
	pub cm_media_id_cm_media:Option<CmMedia>,
}

pub struct CmAdCat {
	/// primary
	/// not nullable 
	pub cm_ad_cat_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub cm_webproject_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub cm_webproject_id_cm_webproject:Option<CmWebproject>,
	/// has many
	pub cm_ad:Option<Vec<CmAd>>,
	/// has many
	pub cm_template_ad_cat:Option<Vec<CmTemplateAdCat>>,
}

pub struct CmBroadcastserver {
	/// primary
	/// not nullable 
	pub cm_broadcastserver_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub cm_webproject_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// not nullable 
	pub ip_address:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub lastsynchronized:Option<NaiveDateTime>,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub cm_webproject_id_cm_webproject:Option<CmWebproject>,
	/// has many
	pub cm_webaccesslog:Option<Vec<CmWebaccesslog>>,
}

pub struct CmChat {
	/// primary
	/// not nullable 
	pub cm_chat_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_table_id:f64,
	pub cm_chattype_id:Option<f64>,
	/// not nullable 
	pub confidentialtype:String,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub description:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub moderationtype:Option<String>,
	/// not nullable 
	pub record_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_table_id_ad_table:Option<AdTable>,
	/// has one
	pub cm_chattype_id_cm_chattype:Option<CmChattype>,
	/// has many
	pub cm_chatentry:Option<Vec<CmChatentry>>,
	/// has many
	pub cm_chatupdate:Option<Vec<CmChatupdate>>,
}

pub struct CmChatentry {
	/// primary
	/// not nullable 
	pub cm_chatentry_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_user_id:Option<f64>,
	pub characterdata:Option<String>,
	/// not nullable 
	pub chatentrytype:String,
	/// not nullable 
	pub cm_chat_id:f64,
	pub cm_chatentrygrandparent_id:Option<f64>,
	pub cm_chatentryparent_id:Option<f64>,
	/// not nullable 
	pub confidentialtype:String,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub moderatorstatus:Option<String>,
	pub subject:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub cm_chat_id_cm_chat:Option<CmChat>,
	/// has one, self referential
	pub cm_chatentryparent_id_cm_chatentry:Option<Box<CmChatentry>>,
	/// has one, self referential
	pub cm_chatentrygrandparent_id_cm_chatentry:Option<Box<CmChatentry>>,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
	/// has many
	pub cm_chatentry:Option<Vec<CmChatentry>>,
}

pub struct CmChattype {
	/// primary
	/// not nullable 
	pub cm_chattype_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_table_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub moderationtype:Option<String>,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_table_id_ad_table:Option<AdTable>,
	/// has many
	pub cm_chat:Option<Vec<CmChat>>,
	/// has many
	pub cm_chattypeupdate:Option<Vec<CmChattypeupdate>>,
}

pub struct CmChattypeupdate {
	/// primary
	/// not nullable 
	pub cm_chattype_id:f64,
	/// primary
	/// not nullable 
	pub ad_user_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isselfservice:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub cm_chattype_id_cm_chattype:Option<CmChattype>,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
}

pub struct CmChatupdate {
	/// primary
	/// not nullable 
	pub cm_chat_id:f64,
	/// primary
	/// not nullable 
	pub ad_user_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isselfservice:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub cm_chat_id_cm_chat:Option<CmChat>,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
}

pub struct CmContainer {
	/// primary
	/// not nullable 
	pub cm_container_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub cm_containerlink_id:Option<f64>,
	pub cm_template_id:Option<f64>,
	/// not nullable 
	pub cm_webproject_id:f64,
	pub containerlinkurl:Option<String>,
	pub containertype:Option<String>,
	pub containerxml:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isindexed:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub issecure:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub issummary:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isvalid:String,
	pub meta_author:Option<String>,
	pub meta_content:Option<String>,
	pub meta_copyright:Option<String>,
	pub meta_description:Option<String>,
	pub meta_keywords:Option<String>,
	pub meta_language:Option<String>,
	pub meta_publisher:Option<String>,
	pub meta_robotstag:Option<String>,
	/// not nullable 
	pub name:String,
	pub notice:Option<String>,
	/// defaults to: 0
	pub priority:Option<f64>,
	pub relativeurl:Option<String>,
	pub structurexml:Option<String>,
	pub title:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub cm_webproject_id_cm_webproject:Option<CmWebproject>,
	/// has one
	pub cm_template_id_cm_template:Option<CmTemplate>,
	/// has one, self referential
	pub cm_containerlink_id_cm_container:Option<Box<CmContainer>>,
	/// has many
	pub cm_accesscontainer:Option<Vec<CmAccesscontainer>>,
	/// has many
	pub cm_container:Option<Vec<CmContainer>>,
	/// has many
	pub cm_container_element:Option<Vec<CmContainerElement>>,
	/// has many
	pub cm_container_trl:Option<Vec<CmContainerTrl>>,
	/// has many
	pub cm_container_url:Option<Vec<CmContainerUrl>>,
	/// has many
	pub cm_containerttable:Option<Vec<CmContainerttable>>,
	/// has many
	pub cm_webproject_domain:Option<Vec<CmWebprojectDomain>>,
}

pub struct CmContainerElement {
	/// primary
	/// not nullable 
	pub cm_container_element_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub cm_container_id:f64,
	pub contenthtml:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isvalid:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub cm_container_id_cm_container:Option<CmContainer>,
	/// has many
	pub cm_container_element_trl:Option<Vec<CmContainerElementTrl>>,
}

pub struct CmContainerElementTrl {
	/// primary
	/// not nullable 
	pub cm_container_element_id:f64,
	/// primary
	/// not nullable 
	pub ad_language:String,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub contenthtml:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub istranslated:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub cm_container_element_id_cm_container_element:Option<CmContainerElement>,
	/// has one
	pub ad_language_ad_language:Option<AdLanguage>,
}

pub struct CmContainerTrl {
	/// primary
	/// not nullable 
	pub cm_container_id:f64,
	/// primary
	/// not nullable 
	pub ad_language:String,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub containerxml:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub istranslated:String,
	pub meta_description:Option<String>,
	pub meta_keywords:Option<String>,
	/// not nullable 
	pub name:String,
	pub structurexml:Option<String>,
	pub title:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub cm_container_id_cm_container:Option<CmContainer>,
	/// has one
	pub ad_language_ad_language:Option<AdLanguage>,
}

pub struct CmContainerUrl {
	/// primary
	/// not nullable 
	pub cm_container_url_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub checked:NaiveDateTime,
	/// not nullable 
	pub cm_container_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub last_result:String,
	/// not nullable 
	pub status:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub cm_container_id_cm_container:Option<CmContainer>,
}

pub struct CmContainerttable {
	/// primary
	/// not nullable 
	pub cm_containerttable_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub cm_container_id:f64,
	/// not nullable 
	pub cm_templatetable_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	pub otherclause:Option<String>,
	pub record_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub whereclause:Option<String>,
	/// has one
	pub cm_container_id_cm_container:Option<CmContainer>,
	/// has one
	pub cm_templatetable_id_cm_templatetable:Option<CmTemplatetable>,
}

pub struct CmCstage {
	/// primary
	/// not nullable 
	pub cm_cstage_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub cm_cstagelink_id:Option<f64>,
	pub cm_template_id:Option<f64>,
	/// not nullable 
	pub cm_webproject_id:f64,
	pub containerlinkurl:Option<String>,
	pub containertype:Option<String>,
	pub containerxml:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isindexed:String,
	/// not nullable 
	pub ismodified:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub issecure:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub issummary:String,
	/// defaults to: 'N'::bpchar
	pub isvalid:Option<String>,
	pub meta_author:Option<String>,
	pub meta_content:Option<String>,
	pub meta_copyright:Option<String>,
	pub meta_description:Option<String>,
	pub meta_keywords:Option<String>,
	pub meta_language:Option<String>,
	pub meta_publisher:Option<String>,
	pub meta_robotstag:Option<String>,
	/// not nullable 
	pub name:String,
	pub notice:Option<String>,
	/// defaults to: 0
	pub priority:Option<f64>,
	pub processing:Option<String>,
	pub relativeurl:Option<String>,
	pub structurexml:Option<String>,
	pub title:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub cm_webproject_id_cm_webproject:Option<CmWebproject>,
	/// has one
	pub cm_template_id_cm_template:Option<CmTemplate>,
	/// has one, self referential
	pub cm_cstagelink_id_cm_cstage:Option<Box<CmCstage>>,
	/// has many
	pub cm_accessstage:Option<Vec<CmAccessstage>>,
	/// has many
	pub cm_cstage:Option<Vec<CmCstage>>,
	/// has many
	pub cm_cstage_element:Option<Vec<CmCstageElement>>,
	/// has many
	pub cm_cstage_trl:Option<Vec<CmCstageTrl>>,
	/// has many
	pub cm_cstagettable:Option<Vec<CmCstagettable>>,
}

pub struct CmCstageElement {
	/// primary
	/// not nullable 
	pub cm_cstage_element_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub cm_cstage_id:f64,
	pub contenthtml:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isvalid:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub cm_cstage_id_cm_cstage:Option<CmCstage>,
	/// has many
	pub cm_cstage_element_trl:Option<Vec<CmCstageElementTrl>>,
}

pub struct CmCstageElementTrl {
	/// primary
	/// not nullable 
	pub cm_cstage_element_id:f64,
	/// primary
	/// not nullable 
	pub ad_language:String,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub contenthtml:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub istranslated:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub cm_cstage_element_id_cm_cstage_element:Option<CmCstageElement>,
	/// has one
	pub ad_language_ad_language:Option<AdLanguage>,
}

pub struct CmCstageTrl {
	/// primary
	/// not nullable 
	pub cm_cstage_id:f64,
	/// primary
	/// not nullable 
	pub ad_language:String,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub containerxml:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub istranslated:String,
	pub meta_description:Option<String>,
	pub meta_keywords:Option<String>,
	/// not nullable 
	pub name:String,
	pub structurexml:Option<String>,
	pub title:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub cm_cstage_id_cm_cstage:Option<CmCstage>,
	/// has one
	pub ad_language_ad_language:Option<AdLanguage>,
}

pub struct CmCstagettable {
	/// primary
	/// not nullable 
	pub cm_cstagettable_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub cm_cstage_id:f64,
	/// not nullable 
	pub cm_templatetable_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	pub otherclause:Option<String>,
	pub record_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub whereclause:Option<String>,
	/// has one
	pub cm_cstage_id_cm_cstage:Option<CmCstage>,
	/// has one
	pub cm_templatetable_id_cm_templatetable:Option<CmTemplatetable>,
}

pub struct CmMedia {
	/// primary
	/// not nullable 
	pub cm_media_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	pub ad_image_id:Option<f64>,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub cm_webproject_id:f64,
	pub contenttext:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub directdeploy:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub issummary:String,
	pub mediatype:Option<String>,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub cm_webproject_id_cm_webproject:Option<CmWebproject>,
	/// has many
	pub cm_accessmedia:Option<Vec<CmAccessmedia>>,
	/// has many
	pub cm_ad:Option<Vec<CmAd>>,
	/// has many
	pub cm_mediadeploy:Option<Vec<CmMediadeploy>>,
	/// has many
	pub cm_webaccesslog:Option<Vec<CmWebaccesslog>>,
}

pub struct CmMediaServer {
	/// primary
	/// not nullable 
	pub cm_media_server_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub cm_webproject_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub folder:Option<String>,
	pub help:Option<String>,
	pub ip_address:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub ispassive:String,
	/// not nullable 
	pub name:String,
	pub password:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub url:Option<String>,
	pub username:Option<String>,
	/// has one
	pub cm_webproject_id_cm_webproject:Option<CmWebproject>,
	/// has many
	pub cm_mediadeploy:Option<Vec<CmMediadeploy>>,
}

pub struct CmMediadeploy {
	/// primary
	/// not nullable 
	pub cm_mediadeploy_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub cm_media_id:f64,
	/// not nullable 
	pub cm_media_server_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isdeployed:String,
	pub lastsynchronized:Option<NaiveDateTime>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub cm_media_server_id_cm_media_server:Option<CmMediaServer>,
	/// has one
	pub cm_media_id_cm_media:Option<CmMedia>,
}

pub struct CmNewschannel {
	/// primary
	/// not nullable 
	pub cm_newschannel_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	pub ad_language:Option<String>,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub cm_webproject_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub description:String,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub link:Option<String>,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_language_ad_language:Option<AdLanguage>,
	/// has one
	pub cm_webproject_id_cm_webproject:Option<CmWebproject>,
	/// has many
	pub cm_accessnewschannel:Option<Vec<CmAccessnewschannel>>,
	/// has many
	pub cm_newsitem:Option<Vec<CmNewsitem>>,
}

pub struct CmNewsitem {
	/// primary
	/// not nullable 
	pub cm_newsitem_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub author:Option<String>,
	/// not nullable 
	pub cm_newschannel_id:f64,
	pub contenthtml:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub linkurl:Option<String>,
	pub pubdate:Option<NaiveDateTime>,
	pub title:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub cm_newschannel_id_cm_newschannel:Option<CmNewschannel>,
}

pub struct CmTemplate {
	/// primary
	/// not nullable 
	pub cm_template_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub cm_webproject_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub elements:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isinclude:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isnews:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub issummary:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isusead:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isvalid:String,
	/// not nullable 
	pub name:String,
	pub processing:Option<String>,
	pub templatexst:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
	/// has one
	pub cm_webproject_id_cm_webproject:Option<CmWebproject>,
	/// has many
	pub cm_container:Option<Vec<CmContainer>>,
	/// has many
	pub cm_cstage:Option<Vec<CmCstage>>,
	/// has many
	pub cm_template_ad_cat:Option<Vec<CmTemplateAdCat>>,
	/// has many
	pub cm_templatetable:Option<Vec<CmTemplatetable>>,
}

pub struct CmTemplateAdCat {
	/// primary
	/// not nullable 
	pub cm_ad_cat_id:f64,
	/// primary
	/// not nullable 
	pub cm_template_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub cm_ad_cat_id_cm_ad_cat:Option<CmAdCat>,
	/// has one
	pub cm_template_id_cm_template:Option<CmTemplate>,
}

pub struct CmTemplatetable {
	/// primary
	/// not nullable 
	pub cm_templatetable_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_table_id:f64,
	/// not nullable 
	pub cm_template_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	pub otherclause:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub whereclause:Option<String>,
	/// has one
	pub cm_template_id_cm_template:Option<CmTemplate>,
	/// has one
	pub ad_table_id_ad_table:Option<AdTable>,
	/// has many
	pub cm_containerttable:Option<Vec<CmContainerttable>>,
	/// has many
	pub cm_cstagettable:Option<Vec<CmCstagettable>>,
}

pub struct CmWebaccesslog {
	/// primary
	/// not nullable 
	pub cm_webaccesslog_id:f64,
	pub acceptlanguage:Option<String>,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_user_id:Option<f64>,
	pub cm_broadcastserver_id:Option<f64>,
	pub cm_media_id:Option<f64>,
	pub cm_webproject_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub filesize:Option<f64>,
	pub hyphen:Option<String>,
	/// not nullable 
	pub ip_address:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub logtype:String,
	pub pageurl:Option<String>,
	/// not nullable 
	pub protocol:String,
	pub referrer:Option<String>,
	pub remote_addr:Option<String>,
	pub remote_host:Option<String>,
	/// not nullable 
	pub requesttype:String,
	/// defaults to: 0
	pub statuscode:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub useragent:Option<String>,
	pub websession:Option<String>,
	/// has one
	pub cm_webproject_id_cm_webproject:Option<CmWebproject>,
	/// has one
	pub cm_broadcastserver_id_cm_broadcastserver:Option<CmBroadcastserver>,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
	/// has one
	pub cm_media_id_cm_media:Option<CmMedia>,
}

pub struct CmWebproject {
	/// primary
	/// not nullable 
	pub cm_webproject_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_treecmc_id:f64,
	/// not nullable 
	pub ad_treecmm_id:f64,
	/// not nullable 
	pub ad_treecms_id:f64,
	/// not nullable 
	pub ad_treecmt_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub meta_author:String,
	/// not nullable 
	pub meta_content:String,
	/// not nullable 
	pub meta_copyright:String,
	/// not nullable 
	pub meta_publisher:String,
	/// not nullable 
	pub meta_robotstag:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_treecmc_id_ad_tree:Option<AdTree>,
	/// has one
	pub ad_treecms_id_ad_tree:Option<AdTree>,
	/// has one
	pub ad_treecmm_id_ad_tree:Option<AdTree>,
	/// has one
	pub ad_treecmt_id_ad_tree:Option<AdTree>,
	/// has many
	pub cm_ad_cat:Option<Vec<CmAdCat>>,
	/// has many
	pub cm_broadcastserver:Option<Vec<CmBroadcastserver>>,
	/// has many
	pub cm_container:Option<Vec<CmContainer>>,
	/// has many
	pub cm_cstage:Option<Vec<CmCstage>>,
	/// has many
	pub cm_media:Option<Vec<CmMedia>>,
	/// has many
	pub cm_media_server:Option<Vec<CmMediaServer>>,
	/// has many
	pub cm_newschannel:Option<Vec<CmNewschannel>>,
	/// has many
	pub cm_template:Option<Vec<CmTemplate>>,
	/// has many
	pub cm_webaccesslog:Option<Vec<CmWebaccesslog>>,
	/// has many
	pub cm_webproject_domain:Option<Vec<CmWebprojectDomain>>,
	/// has many
	pub k_index:Option<Vec<KIndex>>,
	/// has many
	pub k_indexstop:Option<Vec<KIndexstop>>,
}

pub struct CmWebprojectDomain {
	/// primary
	/// not nullable 
	pub cm_webproject_domain_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub cm_container_id:Option<f64>,
	/// not nullable 
	pub cm_webproject_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// not nullable 
	pub fqdn:String,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub cm_webproject_id_cm_webproject:Option<CmWebproject>,
	/// has one
	pub cm_container_id_cm_container:Option<CmContainer>,
}

pub struct CmWikitoken {
	/// primary
	/// not nullable 
	pub cm_wikitoken_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_table_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub macro:Option<String>,
	/// not nullable 
	pub name:String,
	pub selectclause:Option<String>,
	/// not nullable 
	pub tokentype:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub whereclause:Option<String>,
	/// has one
	pub ad_table_id_ad_table:Option<AdTable>,
}

pub struct Company {
	/// primary
	/// not nullable 
	pub company_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: NULL::character varying
	pub description:Option<String>,
	/// defaults to: NULL::character varying
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
}

pub struct Datatype {
	/// primary
	/// not nullable 
	pub datatype_id:f64,
	/// defaults to: NULL::numeric
	pub ad_class_id:Option<f64>,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: NULL::character varying
	pub description:Option<String>,
	/// defaults to: NULL::character varying
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
}

pub struct DatatypeView {
	/// primary
	/// not nullable 
	pub datatype_view_id:f64,
	/// defaults to: NULL::numeric
	pub ad_class_id:Option<f64>,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: NULL::numeric
	pub datatype_id:Option<f64>,
	/// defaults to: NULL::character varying
	pub description:Option<String>,
	/// defaults to: NULL::character varying
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// defaults to: NULL::numeric
	pub view_id:Option<f64>,
}

pub struct DdNetworkdistribution {
	/// primary
	/// not nullable 
	pub dd_networkdistribution_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub copyfrom:Option<String>,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub documentno:Option<String>,
	pub help:Option<String>,
	/// not nullable 
	pub isactive:String,
	pub m_changenotice_id:Option<f64>,
	/// not nullable 
	pub name:String,
	pub processing:Option<String>,
	pub revision:Option<String>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub validfrom:Option<NaiveDateTime>,
	pub validto:Option<NaiveDateTime>,
	/// not nullable 
	pub value:String,
	/// has one
	pub m_changenotice_id_m_changenotice:Option<MChangenotice>,
	/// has many
	pub dd_networkdistributionline:Option<Vec<DdNetworkdistributionline>>,
}

pub struct DdNetworkdistributionline {
	/// primary
	/// not nullable 
	pub dd_networkdistributionline_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub dd_networkdistribution_id:f64,
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub m_shipper_id:f64,
	/// not nullable 
	pub m_warehouse_id:f64,
	/// not nullable 
	pub m_warehousesource_id:f64,
	pub percent:Option<f64>,
	pub priorityno:Option<f64>,
	pub transferttime:Option<f64>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub validfrom:Option<NaiveDateTime>,
	pub validto:Option<NaiveDateTime>,
	/// has one
	pub m_shipper_id_m_shipper:Option<MShipper>,
	/// has one
	pub m_warehousesource_id_m_warehouse:Option<MWarehouse>,
	/// has one
	pub dd_networkdistribution_id_dd_networkdistribution:Option<DdNetworkdistribution>,
	/// has one
	pub m_warehouse_id_m_warehouse:Option<MWarehouse>,
}

pub struct DdOrder {
	/// primary
	/// not nullable 
	pub dd_order_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_orgtrx_id:Option<f64>,
	/// defaults to: (- (1)::numeric)
	pub ad_user_id:Option<f64>,
	pub c_activity_id:Option<f64>,
	/// not nullable 
	pub c_bpartner_id:f64,
	pub c_bpartner_location_id:Option<f64>,
	pub c_campaign_id:Option<f64>,
	pub c_charge_id:Option<f64>,
	/// not nullable 
	pub c_doctype_id:f64,
	pub c_invoice_id:Option<f64>,
	pub c_order_id:Option<f64>,
	pub c_project_id:Option<f64>,
	pub chargeamt:Option<f64>,
	pub createconfirm:Option<String>,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub createfrom:Option<String>,
	pub createpackage:Option<String>,
	pub dateordered:Option<NaiveDateTime>,
	pub dateprinted:Option<NaiveDateTime>,
	pub datepromised:Option<NaiveDateTime>,
	pub datereceived:Option<NaiveDateTime>,
	/// defaults to: 'A'::bpchar
	/// not nullable 
	pub deliveryrule:String,
	/// defaults to: 'P'::bpchar
	/// not nullable 
	pub deliveryviarule:String,
	pub description:Option<String>,
	/// defaults to: 'CO'::bpchar
	/// not nullable 
	pub docaction:String,
	/// defaults to: 'DR'::bpchar
	/// not nullable 
	pub docstatus:String,
	/// not nullable 
	pub documentno:String,
	pub freightamt:Option<f64>,
	/// defaults to: 'I'::bpchar
	/// not nullable 
	pub freightcostrule:String,
	pub generateto:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub isapproved:String,
	pub isdelivered:Option<String>,
	pub isdropship:Option<String>,
	/// not nullable 
	pub isindispute:String,
	/// not nullable 
	pub isintransit:String,
	/// not nullable 
	pub isprinted:String,
	pub isselected:Option<String>,
	/// not nullable 
	pub issotrx:String,
	pub m_shipper_id:Option<f64>,
	/// not nullable 
	pub m_warehouse_id:f64,
	pub nopackages:Option<f64>,
	pub pickdate:Option<NaiveDateTime>,
	pub poreference:Option<String>,
	/// not nullable 
	pub posted:String,
	/// defaults to: '5'::bpchar
	/// not nullable 
	pub priorityrule:String,
	/// not nullable 
	pub processed:String,
	pub processing:Option<String>,
	pub ref_order_id:Option<f64>,
	pub salesrep_id:Option<f64>,
	/// not nullable 
	pub sendemail:String,
	pub shipdate:Option<NaiveDateTime>,
	pub trackingno:Option<String>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub user1_id:Option<f64>,
	pub user2_id:Option<f64>,
	pub volume:Option<f64>,
	pub weight:Option<f64>,
	/// has one
	pub ad_orgtrx_id_ad_org:Option<AdOrg>,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
	/// has one
	pub c_activity_id_c_activity:Option<CActivity>,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub c_bpartner_location_id_c_bpartner_location:Option<CBpartnerLocation>,
	/// has one
	pub c_campaign_id_c_campaign:Option<CCampaign>,
	/// has one
	pub c_charge_id_c_charge:Option<CCharge>,
	/// has one
	pub c_doctype_id_c_doctype:Option<CDoctype>,
	/// has one
	pub c_invoice_id_c_invoice:Option<CInvoice>,
	/// has one
	pub c_order_id_c_order:Option<COrder>,
	/// has one
	pub c_project_id_c_project:Option<CProject>,
	/// has one
	pub m_shipper_id_m_shipper:Option<MShipper>,
	/// has one
	pub m_warehouse_id_m_warehouse:Option<MWarehouse>,
	/// has one
	pub salesrep_id_ad_user:Option<AdUser>,
	/// has one
	pub user1_id_c_elementvalue:Option<CElementvalue>,
	/// has one
	pub user2_id_c_elementvalue:Option<CElementvalue>,
	/// has many
	pub dd_orderline:Option<Vec<DdOrderline>>,
	/// has many
	pub m_movement:Option<Vec<MMovement>>,
	/// has many
	pub pp_mrp:Option<Vec<PpMrp>>,
}

pub struct DdOrderline {
	/// primary
	/// not nullable 
	pub dd_orderline_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_orgtrx_id:Option<f64>,
	pub c_activity_id:Option<f64>,
	pub c_campaign_id:Option<f64>,
	pub c_charge_id:Option<f64>,
	pub c_project_id:Option<f64>,
	/// not nullable 
	pub c_uom_id:f64,
	pub confirmedqty:Option<f64>,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub datedelivered:Option<NaiveDateTime>,
	pub dateordered:Option<NaiveDateTime>,
	pub datepromised:Option<NaiveDateTime>,
	/// not nullable 
	pub dd_order_id:f64,
	pub description:Option<String>,
	pub freightamt:Option<f64>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isdescription:String,
	/// not nullable 
	pub isinvoiced:String,
	/// not nullable 
	pub line:f64,
	pub linenetamt:Option<f64>,
	pub m_attributesetinstance_id:Option<f64>,
	pub m_attributesetinstanceto_id:Option<f64>,
	/// not nullable 
	pub m_locator_id:f64,
	/// not nullable 
	pub m_locatorto_id:f64,
	pub m_product_id:Option<f64>,
	pub pickedqty:Option<f64>,
	/// not nullable 
	pub processed:String,
	pub qtydelivered:Option<f64>,
	/// defaults to: (1)::numeric
	/// not nullable 
	pub qtyentered:f64,
	/// defaults to: (1)::numeric
	pub qtyintransit:Option<f64>,
	/// defaults to: (1)::numeric
	/// not nullable 
	pub qtyordered:f64,
	pub qtyreserved:Option<f64>,
	/// defaults to: (0)::numeric
	pub scrappedqty:Option<f64>,
	pub targetqty:Option<f64>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub user1_id:Option<f64>,
	pub user2_id:Option<f64>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub dd_order_id_dd_order:Option<DdOrder>,
	/// has one
	pub c_charge_id_c_charge:Option<CCharge>,
	/// has one
	pub c_project_id_c_project:Option<CProject>,
	/// has one
	pub c_uom_id_c_uom:Option<CUom>,
	/// has one
	pub m_locatorto_id_m_locator:Option<MLocator>,
	/// has one
	pub m_locator_id_m_locator:Option<MLocator>,
	/// has one
	pub user1_id_c_elementvalue:Option<CElementvalue>,
	/// has one
	pub user2_id_c_elementvalue:Option<CElementvalue>,
	/// has one
	pub ad_orgtrx_id_ad_org:Option<AdOrg>,
	/// has one
	pub c_activity_id_c_activity:Option<CActivity>,
	/// has one
	pub c_campaign_id_c_campaign:Option<CCampaign>,
	/// has many
	pub m_movementline:Option<Vec<MMovementline>>,
	/// has many
	pub pp_mrp:Option<Vec<PpMrp>>,
}

pub struct EdChoice {
	/// primary
	/// not nullable 
	pub ed_choice_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: NULL::character varying
	pub description:Option<String>,
	/// defaults to: NULL::numeric
	pub ed_question_id:Option<f64>,
	/// defaults to: NULL::character varying
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: NULL::bpchar
	pub iscorrect:Option<String>,
	/// not nullable 
	pub text:String,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
}

pub struct EdQuestion {
	/// primary
	/// not nullable 
	pub ed_question_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: NULL::character varying
	pub description:Option<String>,
	/// defaults to: NULL::numeric
	pub ed_questionnaire_id:Option<f64>,
	/// defaults to: NULL::character varying
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub text:Option<String>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
}

pub struct EdQuestionnaire {
	/// primary
	/// not nullable 
	pub ed_questionnaire_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: NULL::character varying
	pub description:Option<String>,
	/// defaults to: NULL::character varying
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub title:String,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
}

pub struct EduCourse {
	/// primary
	/// not nullable 
	pub edu_course_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub edu_department_id:Option<f64>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
}

pub struct EduCourseSubject {
	/// primary
	/// not nullable 
	pub edu_course_subject_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: NULL::character varying
	pub description:Option<String>,
	/// defaults to: NULL::numeric
	pub edu_course_id:Option<f64>,
	/// defaults to: NULL::numeric
	pub edu_semester_id:Option<f64>,
	/// defaults to: NULL::numeric
	pub edu_subject_id:Option<f64>,
	/// defaults to: NULL::numeric
	pub edu_yearlevel_id:Option<f64>,
	/// defaults to: NULL::character varying
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
}

pub struct EduCurriculum {
	/// primary
	/// not nullable 
	pub edu_curriculum_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: NULL::character varying
	pub description:Option<String>,
	/// defaults to: NULL::numeric
	pub edu_semester_id:Option<f64>,
	/// defaults to: NULL::numeric
	pub edu_yearlevel_id:Option<f64>,
	/// defaults to: NULL::character varying
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
}

pub struct EduDepartment {
	/// primary
	/// not nullable 
	pub edu_department_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub dean:Option<String>,
	/// defaults to: NULL::character varying
	pub description:Option<String>,
	/// defaults to: NULL::character varying
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
}

pub struct EduSchedule {
	/// primary
	/// not nullable 
	pub edu_schedule_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: NULL::bpchar
	pub day:Option<String>,
	/// defaults to: NULL::character varying
	pub description:Option<String>,
	/// defaults to: NULL::character varying
	pub help:Option<String>,
	/// defaults to: NULL::character varying
	pub hour:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: NULL::character varying
	pub minute:Option<String>,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
}

pub struct EduScheduleBreakdown {
	/// primary
	/// not nullable 
	pub edu_schedule_breakdown_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: NULL::character varying
	pub description:Option<String>,
	/// defaults to: NULL::numeric
	pub edu_schedule_id:Option<f64>,
	/// defaults to: NULL::character varying
	pub help:Option<String>,
	/// defaults to: NULL::character varying
	pub hour:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: NULL::character varying
	pub minute:Option<String>,
	/// defaults to: NULL::character varying
	pub tohour:Option<String>,
	/// defaults to: '00'::character varying
	pub tominute:Option<String>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
	/// defaults to: NULL::bpchar
	pub weekday:Option<String>,
}

pub struct EduSemester {
	/// primary
	/// not nullable 
	pub edu_semester_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: NULL::character varying
	pub description:Option<String>,
	/// defaults to: NULL::character varying
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
}

pub struct EduSubject {
	/// primary
	/// not nullable 
	pub edu_subject_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: NULL::character varying
	pub description:Option<String>,
	/// defaults to: NULL::numeric
	pub edu_department_id:Option<f64>,
	/// defaults to: NULL::bpchar
	pub haslab:Option<String>,
	/// defaults to: NULL::bpchar
	pub haslect:Option<String>,
	/// defaults to: NULL::character varying
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub labhours:Option<f64>,
	pub lecthours:Option<f64>,
	/// not nullable 
	pub name:String,
	/// defaults to: NULL::numeric
	pub units:Option<f64>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
}

pub struct EduSubjectOfferring {
	/// primary
	/// not nullable 
	pub edu_subject_offerring_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: NULL::character varying
	pub description:Option<String>,
	pub edu_course_id:Option<f64>,
	/// defaults to: NULL::numeric
	pub edu_semester_id:Option<f64>,
	/// defaults to: NULL::numeric
	pub edu_yearlevel_id:Option<f64>,
	/// defaults to: NULL::character varying
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
}

pub struct EduSubjectRequisite {
	/// primary
	/// not nullable 
	pub edu_subject_requisite_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: NULL::character varying
	pub description:Option<String>,
	/// defaults to: NULL::numeric
	pub edu_subject_id:Option<f64>,
	/// defaults to: NULL::character varying
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: NULL::character varying
	pub requisitetype:Option<String>,
	/// defaults to: NULL::character varying
	pub subject_req:Option<String>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
}

pub struct EduSubjectsOffered {
	/// primary
	/// not nullable 
	pub edu_subjects_offered_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: NULL::numeric
	pub c_bpartner_id:Option<f64>,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: NULL::character varying
	pub description:Option<String>,
	/// defaults to: NULL::numeric
	pub edu_subject_id:Option<f64>,
	/// defaults to: NULL::numeric
	pub edu_subject_offerring_id:Option<f64>,
	/// defaults to: NULL::character varying
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	pub isofferred:Option<String>,
	/// not nullable 
	pub name:String,
	pub schedule:Option<String>,
	/// defaults to: NULL::numeric
	pub seqno:Option<f64>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
}

pub struct EduYearlevel {
	/// primary
	/// not nullable 
	pub edu_yearlevel_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: NULL::character varying
	pub description:Option<String>,
	/// defaults to: NULL::character varying
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
}

pub struct Entity {
	/// primary
	/// not nullable 
	pub entity_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: NULL::character varying
	pub description:Option<String>,
	/// defaults to: NULL::character varying
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
}

pub struct ExpFormat {
	/// primary
	/// not nullable 
	pub exp_format_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_table_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	pub processing:Option<String>,
	pub testexportmodel:Option<String>,
	pub testimportmodel:Option<String>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
	/// not nullable 
	pub version:String,
	pub whereclause:Option<String>,
	/// has one
	pub ad_table_id_ad_table:Option<AdTable>,
	/// has many
	pub exp_formatline:Option<Vec<ExpFormatline>>,
}

pub struct ExpFormatline {
	/// primary
	/// not nullable 
	pub exp_formatline_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	pub ad_column_id:Option<f64>,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub dateformat:Option<String>,
	pub description:Option<String>,
	pub exp_embeddedformat_id:Option<f64>,
	pub exp_format_id:Option<f64>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub ismandatory:Option<String>,
	/// defaults to: 'N'::bpchar
	pub ispartuniqueindex:Option<String>,
	/// not nullable 
	pub name:String,
	pub position:Option<f64>,
	/// defaults to: 'E'::bpchar
	/// not nullable 
	pub type_:String,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
	/// has one
	pub exp_format_id_exp_format:Option<ExpFormat>,
	/// has one
	pub ad_column_id_ad_column:Option<AdColumn>,
	/// has one
	pub exp_embeddedformat_id_exp_format:Option<ExpFormat>,
}

pub struct ExpProcessor {
	/// primary
	/// not nullable 
	pub exp_processor_id:f64,
	pub account:Option<String>,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// not nullable 
	pub exp_processor_type_id:f64,
	pub help:Option<String>,
	pub host:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	pub passwordinfo:Option<String>,
	pub port:Option<f64>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
	/// has one
	pub exp_processor_type_id_exp_processor_type:Option<ExpProcessorType>,
	/// has many
	pub ad_replicationstrategy:Option<Vec<AdReplicationstrategy>>,
	/// has many
	pub exp_processorparameter:Option<Vec<ExpProcessorparameter>>,
}

pub struct ExpProcessorType {
	/// primary
	/// not nullable 
	pub exp_processor_type_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub javaclass:String,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
	/// has many
	pub exp_processor:Option<Vec<ExpProcessor>>,
}

pub struct ExpProcessorparameter {
	/// primary
	/// not nullable 
	pub exp_processorparameter_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// not nullable 
	pub exp_processor_id:f64,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	pub parametervalue:Option<String>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
	/// has one
	pub exp_processor_id_exp_processor:Option<ExpProcessor>,
}

pub struct FactAcct {
	/// primary
	/// not nullable 
	pub fact_acct_id:f64,
	pub a_asset_id:Option<f64>,
	/// not nullable 
	pub account_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_orgtrx_id:Option<f64>,
	/// not nullable 
	pub ad_table_id:f64,
	/// defaults to: 0
	/// not nullable 
	pub amtacctcr:f64,
	/// defaults to: 0
	/// not nullable 
	pub amtacctdr:f64,
	/// defaults to: 0
	/// not nullable 
	pub amtsourcecr:f64,
	/// defaults to: 0
	/// not nullable 
	pub amtsourcedr:f64,
	/// not nullable 
	pub c_acctschema_id:f64,
	pub c_activity_id:Option<f64>,
	pub c_bpartner_id:Option<f64>,
	pub c_campaign_id:Option<f64>,
	/// not nullable 
	pub c_currency_id:f64,
	pub c_locfrom_id:Option<f64>,
	pub c_locto_id:Option<f64>,
	pub c_period_id:Option<f64>,
	pub c_project_id:Option<f64>,
	pub c_projectphase_id:Option<f64>,
	pub c_projecttask_id:Option<f64>,
	pub c_salesregion_id:Option<f64>,
	pub c_subacct_id:Option<f64>,
	pub c_tax_id:Option<f64>,
	pub c_uom_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub dateacct:NaiveDateTime,
	/// not nullable 
	pub datetrx:NaiveDateTime,
	pub description:Option<String>,
	pub gl_budget_id:Option<f64>,
	pub gl_category_id:Option<f64>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub line_id:Option<f64>,
	pub m_locator_id:Option<f64>,
	pub m_product_id:Option<f64>,
	/// not nullable 
	pub postingtype:String,
	/// defaults to: 0
	pub qty:Option<f64>,
	/// not nullable 
	pub record_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub user1_id:Option<f64>,
	pub user2_id:Option<f64>,
	pub userelement1_id:Option<f64>,
	pub userelement2_id:Option<f64>,
	/// has one
	pub ad_client_id_ad_client:Option<AdClient>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub c_acctschema_id_c_acctschema:Option<CAcctschema>,
	/// has one
	pub account_id_c_elementvalue:Option<CElementvalue>,
	/// has one
	pub c_period_id_c_period:Option<CPeriod>,
	/// has one
	pub ad_table_id_ad_table:Option<AdTable>,
	/// has one
	pub gl_category_id_gl_category:Option<GlCategory>,
	/// has one
	pub gl_budget_id_gl_budget:Option<GlBudget>,
	/// has one
	pub c_tax_id_c_tax:Option<CTax>,
	/// has one
	pub m_locator_id_m_locator:Option<MLocator>,
	/// has one
	pub c_currency_id_c_currency:Option<CCurrency>,
	/// has one
	pub c_uom_id_c_uom:Option<CUom>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub ad_orgtrx_id_ad_org:Option<AdOrg>,
	/// has one
	pub c_locfrom_id_c_location:Option<CLocation>,
	/// has one
	pub c_locto_id_c_location:Option<CLocation>,
	/// has one
	pub c_salesregion_id_c_salesregion:Option<CSalesregion>,
	/// has one
	pub c_project_id_c_project:Option<CProject>,
	/// has one
	pub c_campaign_id_c_campaign:Option<CCampaign>,
	/// has one
	pub c_activity_id_c_activity:Option<CActivity>,
	/// has one
	pub user1_id_c_elementvalue:Option<CElementvalue>,
	/// has one
	pub user2_id_c_elementvalue:Option<CElementvalue>,
	/// has one
	pub a_asset_id_a_asset:Option<AAsset>,
	/// has one
	pub c_subacct_id_c_subacct:Option<CSubacct>,
	/// has one
	pub c_projectphase_id_c_projectphase:Option<CProjectphase>,
	/// has one
	pub c_projecttask_id_c_projecttask:Option<CProjecttask>,
	/// has many
	pub c_taxdeclarationacct:Option<Vec<CTaxdeclarationacct>>,
	/// has many
	pub t_invoicegl:Option<Vec<TInvoicegl>>,
}

pub struct FactAcctSummary {
	/// not nullable 
	pub account_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: NULL::numeric
	pub ad_orgtrx_id:Option<f64>,
	/// not nullable 
	pub amtacctcr:f64,
	/// not nullable 
	pub amtacctdr:f64,
	/// not nullable 
	pub c_acctschema_id:f64,
	/// defaults to: NULL::numeric
	pub c_activity_id:Option<f64>,
	/// defaults to: NULL::numeric
	pub c_bpartner_id:Option<f64>,
	/// defaults to: NULL::numeric
	pub c_campaign_id:Option<f64>,
	/// defaults to: NULL::numeric
	pub c_locfrom_id:Option<f64>,
	/// defaults to: NULL::numeric
	pub c_locto_id:Option<f64>,
	/// not nullable 
	pub c_period_id:f64,
	/// defaults to: NULL::numeric
	pub c_project_id:Option<f64>,
	/// defaults to: NULL::numeric
	pub c_projectphase_id:Option<f64>,
	/// defaults to: NULL::numeric
	pub c_projecttask_id:Option<f64>,
	/// defaults to: NULL::numeric
	pub c_salesregion_id:Option<f64>,
	/// defaults to: NULL::numeric
	pub c_subacct_id:Option<f64>,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub dateacct:Option<NaiveDateTime>,
	/// defaults to: NULL::numeric
	pub gl_budget_id:Option<f64>,
	/// not nullable 
	pub isactive:String,
	/// defaults to: NULL::numeric
	pub m_product_id:Option<f64>,
	/// not nullable 
	pub pa_reportcube_id:f64,
	/// not nullable 
	pub postingtype:String,
	/// not nullable 
	pub qty:f64,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// defaults to: NULL::numeric
	pub user1_id:Option<f64>,
	/// defaults to: NULL::numeric
	pub user2_id:Option<f64>,
	/// defaults to: NULL::numeric
	pub userelement1_id:Option<f64>,
	/// defaults to: NULL::numeric
	pub userelement2_id:Option<f64>,
}

pub struct GlBudget {
	/// primary
	/// not nullable 
	pub gl_budget_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub budgetstatus:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isprimary:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has many
	pub fact_acct:Option<Vec<FactAcct>>,
	/// has many
	pub gl_budgetcontrol:Option<Vec<GlBudgetcontrol>>,
	/// has many
	pub gl_journal:Option<Vec<GlJournal>>,
	/// has many
	pub i_fajournal:Option<Vec<IFajournal>>,
	/// has many
	pub i_gljournal:Option<Vec<IGljournal>>,
	/// has many
	pub pa_reportcolumn:Option<Vec<PaReportcolumn>>,
	/// has many
	pub pa_reportline:Option<Vec<PaReportline>>,
}

pub struct GlBudgetcontrol {
	/// primary
	/// not nullable 
	pub gl_budgetcontrol_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub budgetcontrolscope:String,
	/// not nullable 
	pub c_acctschema_id:f64,
	/// not nullable 
	pub commitmenttype:String,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// not nullable 
	pub gl_budget_id:f64,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isbeforeapproval:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_acctschema_id_c_acctschema:Option<CAcctschema>,
	/// has one
	pub gl_budget_id_gl_budget:Option<GlBudget>,
}

pub struct GlCategory {
	/// primary
	/// not nullable 
	pub gl_category_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub categorytype:String,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub docbasetype:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isdefault:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has many
	pub a_asset_reval_entry:Option<Vec<AAssetRevalEntry>>,
	/// has many
	pub a_depreciation_entry:Option<Vec<ADepreciationEntry>>,
	/// has many
	pub c_doctype:Option<Vec<CDoctype>>,
	/// has many
	pub fact_acct:Option<Vec<FactAcct>>,
	/// has many
	pub gl_journal:Option<Vec<GlJournal>>,
	/// has many
	pub gl_journalbatch:Option<Vec<GlJournalbatch>>,
	/// has many
	pub i_fajournal:Option<Vec<IFajournal>>,
	/// has many
	pub i_gljournal:Option<Vec<IGljournal>>,
}

pub struct GlDistribution {
	/// primary
	/// not nullable 
	pub gl_distribution_id:f64,
	pub account_id:Option<f64>,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_orgtrx_id:Option<f64>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub anyacct:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub anyactivity:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub anybpartner:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub anycampaign:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub anylocfrom:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub anylocto:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub anyorg:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub anyorgtrx:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub anyproduct:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub anyproject:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub anysalesregion:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub anyuser1:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub anyuser2:String,
	/// not nullable 
	pub c_acctschema_id:f64,
	pub c_activity_id:Option<f64>,
	pub c_bpartner_id:Option<f64>,
	pub c_campaign_id:Option<f64>,
	pub c_doctype_id:Option<f64>,
	pub c_locfrom_id:Option<f64>,
	pub c_locto_id:Option<f64>,
	pub c_project_id:Option<f64>,
	pub c_salesregion_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub iscreatereversal:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isvalid:String,
	pub m_product_id:Option<f64>,
	/// not nullable 
	pub name:String,
	pub org_id:Option<f64>,
	/// not nullable 
	pub percenttotal:f64,
	pub postingtype:Option<String>,
	pub processing:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub user1_id:Option<f64>,
	pub user2_id:Option<f64>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub c_doctype_id_c_doctype:Option<CDoctype>,
	/// has one
	pub c_acctschema_id_c_acctschema:Option<CAcctschema>,
	/// has one
	pub org_id_ad_org:Option<AdOrg>,
	/// has one
	pub account_id_c_elementvalue:Option<CElementvalue>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub c_project_id_c_project:Option<CProject>,
	/// has one
	pub c_campaign_id_c_campaign:Option<CCampaign>,
	/// has one
	pub c_activity_id_c_activity:Option<CActivity>,
	/// has one
	pub c_salesregion_id_c_salesregion:Option<CSalesregion>,
	/// has one
	pub ad_orgtrx_id_ad_org:Option<AdOrg>,
	/// has one
	pub c_locto_id_c_location:Option<CLocation>,
	/// has one
	pub c_locfrom_id_c_location:Option<CLocation>,
	/// has one
	pub user1_id_c_elementvalue:Option<CElementvalue>,
	/// has one
	pub user2_id_c_elementvalue:Option<CElementvalue>,
	/// has many
	pub gl_distributionline:Option<Vec<GlDistributionline>>,
}

pub struct GlDistributionline {
	/// primary
	/// not nullable 
	pub gl_distributionline_id:f64,
	pub account_id:Option<f64>,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_orgtrx_id:Option<f64>,
	pub c_activity_id:Option<f64>,
	pub c_bpartner_id:Option<f64>,
	pub c_campaign_id:Option<f64>,
	pub c_locfrom_id:Option<f64>,
	pub c_locto_id:Option<f64>,
	pub c_project_id:Option<f64>,
	pub c_salesregion_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// not nullable 
	pub gl_distribution_id:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 0
	/// not nullable 
	pub line:f64,
	pub m_product_id:Option<f64>,
	pub org_id:Option<f64>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub overwriteacct:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub overwriteactivity:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub overwritebpartner:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub overwritecampaign:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub overwritelocfrom:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub overwritelocto:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub overwriteorg:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub overwriteorgtrx:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub overwriteproduct:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub overwriteproject:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub overwritesalesregion:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub overwriteuser1:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub overwriteuser2:String,
	/// not nullable 
	pub percent:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub user1_id:Option<f64>,
	pub user2_id:Option<f64>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub gl_distribution_id_gl_distribution:Option<GlDistribution>,
	/// has one
	pub org_id_ad_org:Option<AdOrg>,
	/// has one
	pub account_id_c_elementvalue:Option<CElementvalue>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub c_project_id_c_project:Option<CProject>,
	/// has one
	pub c_campaign_id_c_campaign:Option<CCampaign>,
	/// has one
	pub c_activity_id_c_activity:Option<CActivity>,
	/// has one
	pub c_salesregion_id_c_salesregion:Option<CSalesregion>,
	/// has one
	pub ad_orgtrx_id_ad_org:Option<AdOrg>,
	/// has one
	pub c_locto_id_c_location:Option<CLocation>,
	/// has one
	pub c_locfrom_id_c_location:Option<CLocation>,
	/// has one
	pub user1_id_c_elementvalue:Option<CElementvalue>,
	/// has one
	pub user2_id_c_elementvalue:Option<CElementvalue>,
}

pub struct GlFund {
	/// primary
	/// not nullable 
	pub gl_fund_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: 0
	/// not nullable 
	pub amt:f64,
	/// not nullable 
	pub c_acctschema_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub datefrom:Option<NaiveDateTime>,
	pub dateto:Option<NaiveDateTime>,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_acctschema_id_c_acctschema:Option<CAcctschema>,
	/// has many
	pub gl_fundrestriction:Option<Vec<GlFundrestriction>>,
}

pub struct GlFundrestriction {
	/// primary
	/// not nullable 
	pub gl_fundrestriction_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_elementvalue_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// not nullable 
	pub gl_fund_id:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub gl_fund_id_gl_fund:Option<GlFund>,
	/// has one
	pub c_elementvalue_id_c_elementvalue:Option<CElementvalue>,
}

pub struct GlJournal {
	/// primary
	/// not nullable 
	pub gl_journal_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_acctschema_id:f64,
	/// not nullable 
	pub c_conversiontype_id:f64,
	pub c_currency_id:Option<f64>,
	/// not nullable 
	pub c_doctype_id:f64,
	/// not nullable 
	pub c_period_id:f64,
	/// defaults to: 0
	pub controlamt:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 0
	/// not nullable 
	pub currencyrate:f64,
	/// not nullable 
	pub dateacct:NaiveDateTime,
	/// not nullable 
	pub datedoc:NaiveDateTime,
	/// not nullable 
	pub description:String,
	/// not nullable 
	pub docaction:String,
	/// not nullable 
	pub docstatus:String,
	/// not nullable 
	pub documentno:String,
	pub gl_budget_id:Option<f64>,
	/// not nullable 
	pub gl_category_id:f64,
	pub gl_journalbatch_id:Option<f64>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isapproved:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isprinted:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub posted:String,
	/// not nullable 
	pub postingtype:String,
	/// defaults to: 'N'::bpchar
	pub processed:Option<String>,
	pub processing:Option<String>,
	pub reversal_id:Option<f64>,
	/// defaults to: 0
	/// not nullable 
	pub totalcr:f64,
	/// defaults to: 0
	/// not nullable 
	pub totaldr:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_acctschema_id_c_acctschema:Option<CAcctschema>,
	/// has one
	pub c_doctype_id_c_doctype:Option<CDoctype>,
	/// has one
	pub gl_budget_id_gl_budget:Option<GlBudget>,
	/// has one
	pub gl_category_id_gl_category:Option<GlCategory>,
	/// has one
	pub c_period_id_c_period:Option<CPeriod>,
	/// has one
	pub c_currency_id_c_currency:Option<CCurrency>,
	/// has one
	pub gl_journalbatch_id_gl_journalbatch:Option<GlJournalbatch>,
	/// has one
	pub c_conversiontype_id_c_conversiontype:Option<CConversiontype>,
	/// has one, self referential
	pub reversal_id_gl_journal:Option<Box<GlJournal>>,
	/// has many
	pub c_revenuerecognition_run:Option<Vec<CRevenuerecognitionRun>>,
	/// has many
	pub gl_journal:Option<Vec<GlJournal>>,
	/// has many
	pub gl_journalline:Option<Vec<GlJournalline>>,
	/// has many
	pub i_fajournal:Option<Vec<IFajournal>>,
	/// has many
	pub i_gljournal:Option<Vec<IGljournal>>,
}

pub struct GlJournalbatch {
	/// primary
	/// not nullable 
	pub gl_journalbatch_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub c_currency_id:Option<f64>,
	/// not nullable 
	pub c_doctype_id:f64,
	pub c_period_id:Option<f64>,
	/// defaults to: 0
	pub controlamt:Option<f64>,
	pub copyfrom:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub dateacct:Option<NaiveDateTime>,
	pub datedoc:Option<NaiveDateTime>,
	/// not nullable 
	pub description:String,
	/// not nullable 
	pub docaction:String,
	/// not nullable 
	pub docstatus:String,
	/// not nullable 
	pub documentno:String,
	pub gl_category_id:Option<f64>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub isapproved:Option<String>,
	/// not nullable 
	pub postingtype:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processing:String,
	pub reversal_id:Option<f64>,
	/// defaults to: 0
	/// not nullable 
	pub totalcr:f64,
	/// defaults to: 0
	/// not nullable 
	pub totaldr:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub gl_category_id_gl_category:Option<GlCategory>,
	/// has one
	pub c_period_id_c_period:Option<CPeriod>,
	/// has one
	pub c_currency_id_c_currency:Option<CCurrency>,
	/// has one
	pub c_doctype_id_c_doctype:Option<CDoctype>,
	/// has one, self referential
	pub reversal_id_gl_journalbatch:Option<Box<GlJournalbatch>>,
	/// has many
	pub a_asset_addition:Option<Vec<AAssetAddition>>,
	/// has many
	pub c_recurring:Option<Vec<CRecurring>>,
	/// has many
	pub c_recurring_run:Option<Vec<CRecurringRun>>,
	/// has many
	pub gl_journal:Option<Vec<GlJournal>>,
	/// has many
	pub gl_journalbatch:Option<Vec<GlJournalbatch>>,
	/// has many
	pub i_fajournal:Option<Vec<IFajournal>>,
	/// has many
	pub i_gljournal:Option<Vec<IGljournal>>,
}

pub struct GlJournalline {
	/// primary
	/// not nullable 
	pub gl_journalline_id:f64,
	pub a_asset_group_id:Option<f64>,
	pub a_asset_id:Option<f64>,
	/// defaults to: 'N'::bpchar
	pub a_createasset:Option<String>,
	/// defaults to: 'N'::bpchar
	pub a_processed:Option<String>,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: 0
	/// not nullable 
	pub amtacctcr:f64,
	/// defaults to: 0
	/// not nullable 
	pub amtacctdr:f64,
	/// defaults to: 0
	/// not nullable 
	pub amtsourcecr:f64,
	/// defaults to: 0
	/// not nullable 
	pub amtsourcedr:f64,
	/// not nullable 
	pub c_conversiontype_id:f64,
	/// not nullable 
	pub c_currency_id:f64,
	pub c_uom_id:Option<f64>,
	/// not nullable 
	pub c_validcombination_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 0
	/// not nullable 
	pub currencyrate:f64,
	/// not nullable 
	pub dateacct:NaiveDateTime,
	pub description:Option<String>,
	/// not nullable 
	pub gl_journal_id:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isgenerated:String,
	/// not nullable 
	pub line:f64,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	/// defaults to: 0
	pub qty:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_client_id_ad_client:Option<AdClient>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub gl_journal_id_gl_journal:Option<GlJournal>,
	/// has one
	pub c_currency_id_c_currency:Option<CCurrency>,
	/// has one
	pub c_uom_id_c_uom:Option<CUom>,
	/// has one
	pub c_validcombination_id_c_validcombination:Option<CValidcombination>,
	/// has one
	pub c_conversiontype_id_c_conversiontype:Option<CConversiontype>,
	/// has one
	pub a_asset_group_id_a_asset_group:Option<AAssetGroup>,
	/// has one
	pub a_asset_id_a_asset:Option<AAsset>,
	/// has many
	pub i_fajournal:Option<Vec<IFajournal>>,
	/// has many
	pub i_gljournal:Option<Vec<IGljournal>>,
}

pub struct HrAttribute {
	/// primary
	/// not nullable 
	pub hr_attribute_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_rule_id:Option<f64>,
	pub amount:Option<f64>,
	pub c_bpartner_id:Option<f64>,
	pub columntype:Option<String>,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub hr_attribute_acct:Option<f64>,
	/// not nullable 
	pub hr_concept_id:f64,
	pub hr_department_id:Option<f64>,
	pub hr_employee_id:Option<f64>,
	pub hr_job_id:Option<f64>,
	pub hr_payroll_id:Option<f64>,
	/// not nullable 
	pub isactive:String,
	pub isprinted:Option<String>,
	pub maxvalue:Option<f64>,
	pub minvalue:Option<f64>,
	pub qty:Option<f64>,
	pub servicedate:Option<NaiveDateTime>,
	pub textmsg:Option<String>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub validfrom:NaiveDateTime,
	pub validto:Option<NaiveDateTime>,
	/// has one
	pub ad_rule_id_ad_rule:Option<AdRule>,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub hr_concept_id_hr_concept:Option<HrConcept>,
	/// has one
	pub hr_department_id_hr_department:Option<HrDepartment>,
	/// has one
	pub hr_employee_id_hr_employee:Option<HrEmployee>,
	/// has one
	pub hr_job_id_hr_job:Option<HrJob>,
	/// has one
	pub hr_payroll_id_hr_payroll:Option<HrPayroll>,
}

pub struct HrConcept {
	/// primary
	/// not nullable 
	pub hr_concept_id:f64,
	pub accountsign:Option<String>,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: NULL::numeric
	pub ad_reference_id:Option<f64>,
	/// not nullable 
	pub columntype:String,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub hr_concept_category_id:Option<f64>,
	pub hr_department_id:Option<f64>,
	pub hr_job_id:Option<f64>,
	pub hr_payroll_id:Option<f64>,
	/// not nullable 
	pub isactive:String,
	pub isdefault:Option<String>,
	pub isemployee:Option<String>,
	pub ispaid:Option<String>,
	pub isprinted:Option<String>,
	pub isreadwrite:Option<String>,
	pub isreceipt:Option<String>,
	pub isregistered:Option<String>,
	/// not nullable 
	pub name:String,
	/// defaults to: NULL::numeric
	pub seqno:Option<f64>,
	/// not nullable 
	pub type_:String,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub validfrom:Option<NaiveDateTime>,
	pub validto:Option<NaiveDateTime>,
	pub value:Option<String>,
	/// has one
	pub hr_concept_category_id_hr_concept_category:Option<HrConceptCategory>,
	/// has one
	pub hr_department_id_hr_department:Option<HrDepartment>,
	/// has one
	pub hr_job_id_hr_job:Option<HrJob>,
	/// has one
	pub hr_payroll_id_hr_payroll:Option<HrPayroll>,
	/// has one
	pub ad_reference_id_ad_reference:Option<AdReference>,
	/// has many
	pub hr_attribute:Option<Vec<HrAttribute>>,
	/// has many
	pub hr_concept_acct:Option<Vec<HrConceptAcct>>,
	/// has many
	pub hr_movement:Option<Vec<HrMovement>>,
	/// has many
	pub hr_payrollconcept:Option<Vec<HrPayrollconcept>>,
}

pub struct HrConceptAcct {
	/// primary
	/// not nullable 
	pub hr_concept_acct_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_acctschema_id:f64,
	pub c_bp_group_id:Option<f64>,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub hr_concept_id:f64,
	/// not nullable 
	pub hr_expense_acct:f64,
	/// not nullable 
	pub hr_revenue_acct:f64,
	/// not nullable 
	pub isactive:String,
	pub isbalancing:Option<String>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub user1_id:Option<f64>,
	pub user2_id:Option<f64>,
	/// has one
	pub c_bp_group_id_c_bp_group:Option<CBpGroup>,
	/// has one
	pub user1_id_c_elementvalue:Option<CElementvalue>,
	/// has one
	pub hr_concept_id_hr_concept:Option<HrConcept>,
	/// has one
	pub c_acctschema_id_c_acctschema:Option<CAcctschema>,
}

pub struct HrConceptCategory {
	/// primary
	/// not nullable 
	pub hr_concept_category_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub hr_concept_acct:Option<f64>,
	/// not nullable 
	pub isactive:String,
	pub isdefault:Option<String>,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub value:Option<String>,
	/// has many
	pub hr_concept:Option<Vec<HrConcept>>,
	/// has many
	pub hr_movement:Option<Vec<HrMovement>>,
}

pub struct HrContract {
	/// primary
	/// not nullable 
	pub hr_contract_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub c_bpartner_id:Option<f64>,
	pub c_campaign_id:Option<f64>,
	pub c_project_id:Option<f64>,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	pub netdays:Option<f64>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub validfrom:Option<NaiveDateTime>,
	pub validto:Option<NaiveDateTime>,
	pub value:Option<String>,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub c_campaign_id_c_campaign:Option<CCampaign>,
	/// has one
	pub c_project_id_c_project:Option<CProject>,
	/// has many
	pub hr_payroll:Option<Vec<HrPayroll>>,
}

pub struct HrDepartment {
	/// primary
	/// not nullable 
	pub hr_department_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub c_activity_id:Option<f64>,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub value:Option<String>,
	/// has one
	pub c_activity_id_c_activity:Option<CActivity>,
	/// has many
	pub hr_attribute:Option<Vec<HrAttribute>>,
	/// has many
	pub hr_concept:Option<Vec<HrConcept>>,
	/// has many
	pub hr_employee:Option<Vec<HrEmployee>>,
	/// has many
	pub hr_job:Option<Vec<HrJob>>,
	/// has many
	pub hr_list:Option<Vec<HrList>>,
	/// has many
	pub hr_movement:Option<Vec<HrMovement>>,
	/// has many
	pub hr_process:Option<Vec<HrProcess>>,
}

pub struct HrEmployee {
	/// primary
	/// not nullable 
	pub hr_employee_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub c_activity_id:Option<f64>,
	/// not nullable 
	pub c_bpartner_id:f64,
	pub code:Option<String>,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub enddate:Option<NaiveDateTime>,
	/// not nullable 
	pub hr_department_id:f64,
	/// not nullable 
	pub hr_job_id:f64,
	pub hr_payroll_id:Option<f64>,
	/// defaults to: NULL::character varying
	pub imageurl:Option<String>,
	/// not nullable 
	pub isactive:String,
	pub name:Option<String>,
	pub name2:Option<String>,
	pub nationalcode:Option<String>,
	pub sscode:Option<String>,
	/// not nullable 
	pub startdate:NaiveDateTime,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_activity_id_c_activity:Option<CActivity>,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub hr_department_id_hr_department:Option<HrDepartment>,
	/// has one
	pub hr_job_id_hr_job:Option<HrJob>,
	/// has one
	pub hr_payroll_id_hr_payroll:Option<HrPayroll>,
	/// has many
	pub hr_attribute:Option<Vec<HrAttribute>>,
	/// has many
	pub hr_list:Option<Vec<HrList>>,
	/// has many
	pub hr_process:Option<Vec<HrProcess>>,
}

pub struct HrJob {
	/// primary
	/// not nullable 
	pub hr_job_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub hr_department_id:Option<f64>,
	/// not nullable 
	pub isactive:String,
	pub isparent:Option<String>,
	pub jobcant:Option<f64>,
	/// not nullable 
	pub name:String,
	pub next_job_id:Option<f64>,
	pub supervisor_id:Option<f64>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub value:Option<String>,
	/// has one
	pub hr_department_id_hr_department:Option<HrDepartment>,
	/// has one, self referential
	pub next_job_id_hr_job:Option<Box<HrJob>>,
	/// has one
	pub supervisor_id_ad_user:Option<AdUser>,
	/// has many
	pub hr_attribute:Option<Vec<HrAttribute>>,
	/// has many
	pub hr_concept:Option<Vec<HrConcept>>,
	/// has many
	pub hr_employee:Option<Vec<HrEmployee>>,
	/// has many
	pub hr_job:Option<Vec<HrJob>>,
	/// has many
	pub hr_movement:Option<Vec<HrMovement>>,
	/// has many
	pub hr_process:Option<Vec<HrProcess>>,
}

pub struct HrList {
	/// primary
	/// not nullable 
	pub hr_list_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub hr_department_id:Option<f64>,
	pub hr_employee_id:Option<f64>,
	pub hr_listtype_id:Option<f64>,
	pub hr_payroll_id:Option<f64>,
	/// not nullable 
	pub isactive:String,
	pub isemployee:Option<String>,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub validfrom:Option<NaiveDateTime>,
	pub value:Option<String>,
	/// has one
	pub hr_department_id_hr_department:Option<HrDepartment>,
	/// has one
	pub hr_employee_id_hr_employee:Option<HrEmployee>,
	/// has one
	pub hr_listtype_id_hr_listtype:Option<HrListtype>,
	/// has one
	pub hr_payroll_id_hr_payroll:Option<HrPayroll>,
	/// has many
	pub hr_listversion:Option<Vec<HrListversion>>,
}

pub struct HrListline {
	/// primary
	/// not nullable 
	pub hr_listline_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub col_1:Option<f64>,
	pub col_2:Option<f64>,
	pub col_3:Option<f64>,
	pub col_4:Option<f64>,
	pub col_5:Option<f64>,
	pub col_6:Option<f64>,
	pub col_7:Option<f64>,
	pub col_8:Option<f64>,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub hr_listversion_id:Option<f64>,
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub maxvalue:f64,
	/// not nullable 
	pub minvalue:f64,
	pub name:Option<String>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub hr_listversion_id_hr_listversion:Option<HrListversion>,
}

pub struct HrListtype {
	/// primary
	/// not nullable 
	pub hr_listtype_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub value:Option<String>,
	/// has many
	pub hr_list:Option<Vec<HrList>>,
}

pub struct HrListversion {
	/// primary
	/// not nullable 
	pub hr_listversion_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// not nullable 
	pub hr_list_id:f64,
	pub hr_listbase_id:Option<f64>,
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub validfrom:NaiveDateTime,
	/// not nullable 
	pub validto:NaiveDateTime,
	/// has one
	pub hr_listbase_id_hr_list:Option<HrList>,
	/// has one
	pub hr_list_id_hr_list:Option<HrList>,
	/// has many
	pub hr_listline:Option<Vec<HrListline>>,
}

pub struct HrMovement {
	/// primary
	/// not nullable 
	pub hr_movement_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_orgtrx_id:Option<f64>,
	pub ad_rule_id:Option<f64>,
	pub amount:Option<f64>,
	pub c_activity_id:Option<f64>,
	pub c_bpartner_id:Option<f64>,
	pub c_campaign_id:Option<f64>,
	pub c_project_id:Option<f64>,
	pub c_projectphase_id:Option<f64>,
	pub c_projecttask_id:Option<f64>,
	pub columntype:Option<String>,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub hr_concept_category_id:Option<f64>,
	/// not nullable 
	pub hr_concept_id:f64,
	pub hr_department_id:Option<f64>,
	pub hr_job_id:Option<f64>,
	pub hr_process_id:Option<f64>,
	/// not nullable 
	pub isactive:String,
	pub isprinted:Option<String>,
	pub isregistered:Option<String>,
	pub pp_cost_collector_id:Option<f64>,
	/// not nullable 
	pub processed:String,
	pub qty:Option<f64>,
	pub servicedate:Option<NaiveDateTime>,
	pub textmsg:Option<String>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub user1_id:Option<f64>,
	pub user2_id:Option<f64>,
	/// not nullable 
	pub validfrom:NaiveDateTime,
	pub validto:Option<NaiveDateTime>,
	/// has one
	pub hr_concept_category_id_hr_concept_category:Option<HrConceptCategory>,
	/// has one
	pub hr_process_id_hr_process:Option<HrProcess>,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub hr_concept_id_hr_concept:Option<HrConcept>,
	/// has one
	pub hr_department_id_hr_department:Option<HrDepartment>,
	/// has one
	pub hr_job_id_hr_job:Option<HrJob>,
	/// has one
	pub ad_rule_id_ad_rule:Option<AdRule>,
	/// has one
	pub c_activity_id_c_activity:Option<CActivity>,
	/// has one
	pub c_campaign_id_c_campaign:Option<CCampaign>,
	/// has one
	pub ad_orgtrx_id_ad_org:Option<AdOrg>,
	/// has one
	pub c_projectphase_id_c_projectphase:Option<CProjectphase>,
	/// has one
	pub c_projecttask_id_c_projecttask:Option<CProjecttask>,
	/// has one
	pub c_project_id_c_project:Option<CProject>,
	/// has one
	pub user1_id_c_elementvalue:Option<CElementvalue>,
	/// has one
	pub user2_id_c_elementvalue:Option<CElementvalue>,
	/// has one
	pub pp_cost_collector_id_pp_cost_collector:Option<PpCostCollector>,
}

pub struct HrPayroll {
	/// primary
	/// not nullable 
	pub hr_payroll_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_printformat_id:Option<f64>,
	pub c_charge_id:Option<f64>,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// not nullable 
	pub hr_contract_id:f64,
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub paymentrule:String,
	pub processed:Option<String>,
	pub processing:Option<String>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub value:Option<String>,
	/// has one
	pub ad_printformat_id_ad_printformat:Option<AdPrintformat>,
	/// has one
	pub c_charge_id_c_charge:Option<CCharge>,
	/// has one
	pub hr_contract_id_hr_contract:Option<HrContract>,
	/// has many
	pub hr_attribute:Option<Vec<HrAttribute>>,
	/// has many
	pub hr_concept:Option<Vec<HrConcept>>,
	/// has many
	pub hr_employee:Option<Vec<HrEmployee>>,
	/// has many
	pub hr_list:Option<Vec<HrList>>,
	/// has many
	pub hr_payrollconcept:Option<Vec<HrPayrollconcept>>,
	/// has many
	pub hr_period:Option<Vec<HrPeriod>>,
	/// has many
	pub hr_process:Option<Vec<HrProcess>>,
	/// has many
	pub hr_year:Option<Vec<HrYear>>,
}

pub struct HrPayrollconcept {
	/// primary
	/// not nullable 
	pub hr_payrollconcept_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_rule_id:Option<f64>,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub hr_concept_id:f64,
	/// not nullable 
	pub hr_payroll_id:f64,
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	pub isdisplayed:Option<String>,
	pub isinclude:Option<String>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isprinted:String,
	pub name:Option<String>,
	pub seqno:Option<f64>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_rule_id_ad_rule:Option<AdRule>,
	/// has one
	pub hr_concept_id_hr_concept:Option<HrConcept>,
	/// has one
	pub hr_payroll_id_hr_payroll:Option<HrPayroll>,
}

pub struct HrPeriod {
	/// primary
	/// not nullable 
	pub hr_period_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_period_id:f64,
	/// not nullable 
	pub c_year_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub dateacct:NaiveDateTime,
	pub description:Option<String>,
	pub enddate:Option<NaiveDateTime>,
	/// not nullable 
	pub hr_payroll_id:f64,
	pub hr_year_id:Option<f64>,
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	pub periodaction:Option<String>,
	/// not nullable 
	pub periodno:f64,
	pub periodstatus:Option<String>,
	pub processed:Option<String>,
	pub processing:Option<String>,
	/// not nullable 
	pub startdate:NaiveDateTime,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_period_id_c_period:Option<CPeriod>,
	/// has one
	pub c_year_id_c_year:Option<CYear>,
	/// has one
	pub hr_payroll_id_hr_payroll:Option<HrPayroll>,
	/// has one
	pub hr_year_id_hr_year:Option<HrYear>,
	/// has many
	pub hr_process:Option<Vec<HrProcess>>,
}

pub struct HrProcess {
	/// primary
	/// defaults to: (- (1)::numeric)
	/// not nullable 
	pub hr_process_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_printformat_id:Option<f64>,
	pub ad_workflow_id:Option<f64>,
	pub c_bpartner_id:Option<f64>,
	pub c_charge_id:Option<f64>,
	pub c_doctype_id:Option<f64>,
	/// not nullable 
	pub c_doctypetarget_id:f64,
	pub c_payselection_id:Option<f64>,
	pub columnsql:Option<String>,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub dateacct:NaiveDateTime,
	/// defaults to: 'CO'::bpchar
	/// not nullable 
	pub docaction:String,
	/// defaults to: 'DR'::character varying
	/// not nullable 
	pub docstatus:String,
	pub documentno:Option<String>,
	pub hr_department_id:Option<f64>,
	pub hr_employee_id:Option<f64>,
	pub hr_job_id:Option<f64>,
	/// not nullable 
	pub hr_payroll_id:f64,
	pub hr_period_id:Option<f64>,
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub posted:String,
	/// not nullable 
	pub processed:String,
	pub processing:Option<String>,
	pub reversal_id:Option<f64>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub hr_job_id_hr_job:Option<HrJob>,
	/// has one
	pub c_doctype_id_c_doctype:Option<CDoctype>,
	/// has one
	pub c_payselection_id_c_payselection:Option<CPayselection>,
	/// has one
	pub hr_employee_id_hr_employee:Option<HrEmployee>,
	/// has one
	pub c_doctypetarget_id_c_doctype:Option<CDoctype>,
	/// has one
	pub hr_department_id_hr_department:Option<HrDepartment>,
	/// has one
	pub hr_payroll_id_hr_payroll:Option<HrPayroll>,
	/// has one
	pub hr_period_id_hr_period:Option<HrPeriod>,
	/// has one
	pub ad_printformat_id_ad_printformat:Option<AdPrintformat>,
	/// has one
	pub ad_workflow_id_ad_workflow:Option<AdWorkflow>,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub c_charge_id_c_charge:Option<CCharge>,
	/// has one, self referential
	pub reversal_id_hr_process:Option<Box<HrProcess>>,
	/// has many
	pub hr_movement:Option<Vec<HrMovement>>,
	/// has many
	pub hr_process:Option<Vec<HrProcess>>,
}

pub struct HrYear {
	/// primary
	/// not nullable 
	pub hr_year_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_year_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub hr_payroll_id:f64,
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub netdays:f64,
	pub processed:Option<String>,
	pub processing:Option<String>,
	/// not nullable 
	pub qty:f64,
	/// not nullable 
	pub startdate:NaiveDateTime,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_year_id_c_year:Option<CYear>,
	/// has one
	pub hr_payroll_id_hr_payroll:Option<HrPayroll>,
	/// has many
	pub hr_period:Option<Vec<HrPeriod>>,
}

pub struct IAsset {
	/// primary
	/// not nullable 
	pub i_asset_id:f64,
	pub a_accumdepreciation_acct:Option<f64>,
	pub a_accumulated_depr:Option<f64>,
	pub a_asset_acct:Option<f64>,
	pub a_asset_cost:Option<f64>,
	pub a_asset_group_id:Option<f64>,
	pub a_asset_id:Option<f64>,
	pub a_asset_life_current_year:Option<f64>,
	pub a_asset_life_years:Option<f64>,
	pub a_asset_spread_type:Option<String>,
	pub a_base_amount:Option<f64>,
	pub a_calc_accumulated_depr:Option<f64>,
	pub a_curr_dep_exp:Option<f64>,
	pub a_current_period:Option<f64>,
	pub a_depreciation_acct:Option<f64>,
	pub a_depreciation_calc_type:Option<String>,
	pub a_depreciation_manual_amount:Option<f64>,
	pub a_depreciation_manual_period:Option<String>,
	pub a_depreciation_table_header_id:Option<f64>,
	pub a_depreciation_variable_perc:Option<f64>,
	pub a_disposal_loss:Option<String>,
	pub a_disposal_revenue:Option<String>,
	pub a_life_period:Option<f64>,
	/// defaults to: (0)::numeric
	pub a_parent_asset_id:Option<f64>,
	pub a_period_end:Option<f64>,
	pub a_period_posted:Option<f64>,
	pub a_period_start:Option<f64>,
	pub a_prior_year_accumulated_depr:Option<f64>,
	pub a_qty_current:Option<f64>,
	pub a_qty_original:Option<f64>,
	pub a_reval_accumdep_offset_cur:Option<String>,
	pub a_reval_accumdep_offset_prior:Option<String>,
	pub a_reval_cal_method:Option<String>,
	pub a_reval_cost_offset:Option<String>,
	pub a_reval_cost_offset_prior:Option<String>,
	pub a_reval_depexp_offset:Option<String>,
	pub a_salvage_value:Option<f64>,
	pub a_split_percent:Option<f64>,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_user_id:Option<f64>,
	pub assetdepreciationdate:Option<NaiveDateTime>,
	pub assetdisposaldate:Option<NaiveDateTime>,
	pub assetmarketvalueamt:Option<f64>,
	pub assetservicedate:Option<NaiveDateTime>,
	pub c_acctschema_id:Option<f64>,
	pub c_bpartner_id:Option<f64>,
	pub c_bpartner_location_id:Option<f64>,
	pub c_location_id:Option<f64>,
	pub conventiontype:Option<String>,
	pub created:Option<NaiveDateTime>,
	pub createdby:Option<f64>,
	pub depreciationtype:Option<String>,
	pub description:Option<String>,
	pub guaranteedate:Option<NaiveDateTime>,
	pub help:Option<String>,
	pub i_errormsg:Option<String>,
	pub i_isimported:Option<String>,
	pub isactive:Option<String>,
	pub isdepreciated:Option<String>,
	pub isdisposed:Option<String>,
	pub isfullydepreciated:Option<String>,
	pub isinposession:Option<String>,
	pub isowned:Option<String>,
	pub lifeuseunits:Option<f64>,
	pub locationcomment:Option<String>,
	pub lot:Option<String>,
	pub m_attributesetinstance_id:Option<f64>,
	pub m_locator_id:Option<f64>,
	pub m_product_id:Option<f64>,
	pub name:Option<String>,
	pub postingtype:Option<String>,
	pub processed:Option<String>,
	pub processing:Option<String>,
	pub serno:Option<String>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub uselifemonths:Option<f64>,
	pub uselifeyears:Option<f64>,
	pub useunits:Option<f64>,
	pub value:Option<String>,
	pub versionno:Option<String>,
	/// has one
	pub a_asset_group_id_a_asset_group:Option<AAssetGroup>,
	/// has one
	pub a_depreciation_table_header_id_a_depreciation_table_header:Option<ADepreciationTableHeader>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub m_locator_id_m_locator:Option<MLocator>,
	/// has one
	pub c_acctschema_id_c_acctschema:Option<CAcctschema>,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub c_bpartner_location_id_c_bpartner_location:Option<CBpartnerLocation>,
	/// has one
	pub c_location_id_c_location:Option<CLocation>,
}

pub struct IBankstatement {
	/// primary
	/// not nullable 
	pub i_bankstatement_id:f64,
	pub ad_client_id:Option<f64>,
	pub ad_org_id:Option<f64>,
	pub bankaccountno:Option<String>,
	pub bpartnervalue:Option<String>,
	pub c_bankaccount_id:Option<f64>,
	pub c_bankstatement_id:Option<f64>,
	pub c_bankstatementline_id:Option<f64>,
	pub c_bpartner_id:Option<f64>,
	pub c_charge_id:Option<f64>,
	pub c_currency_id:Option<f64>,
	pub c_invoice_id:Option<f64>,
	pub c_payment_id:Option<f64>,
	/// defaults to: 0
	pub chargeamt:Option<f64>,
	pub chargename:Option<String>,
	/// defaults to: now()
	pub created:Option<NaiveDateTime>,
	pub createdby:Option<f64>,
	pub createpayment:Option<String>,
	pub dateacct:Option<NaiveDateTime>,
	pub description:Option<String>,
	/// defaults to: 0
	pub eftamt:Option<f64>,
	pub eftcheckno:Option<String>,
	pub eftcurrency:Option<String>,
	pub eftmemo:Option<String>,
	pub eftpayee:Option<String>,
	pub eftpayeeaccount:Option<String>,
	pub eftreference:Option<String>,
	pub eftstatementdate:Option<NaiveDateTime>,
	pub eftstatementlinedate:Option<NaiveDateTime>,
	pub eftstatementreference:Option<String>,
	pub efttrxid:Option<String>,
	pub efttrxtype:Option<String>,
	pub eftvalutadate:Option<NaiveDateTime>,
	pub i_errormsg:Option<String>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub i_isimported:String,
	/// defaults to: 0
	pub interestamt:Option<f64>,
	pub invoicedocumentno:Option<String>,
	/// defaults to: 'Y'::bpchar
	pub isactive:Option<String>,
	pub iso_code:Option<String>,
	/// defaults to: 'N'::bpchar
	pub isreversal:Option<String>,
	pub line:Option<f64>,
	pub linedescription:Option<String>,
	pub matchstatement:Option<String>,
	pub memo:Option<String>,
	pub name:Option<String>,
	pub paymentdocumentno:Option<String>,
	/// defaults to: 'N'::bpchar
	pub processed:Option<String>,
	pub processing:Option<String>,
	pub referenceno:Option<String>,
	pub routingno:Option<String>,
	pub statementdate:Option<NaiveDateTime>,
	pub statementlinedate:Option<NaiveDateTime>,
	/// defaults to: 0
	pub stmtamt:Option<f64>,
	/// defaults to: 0
	pub trxamt:Option<f64>,
	pub trxtype:Option<String>,
	/// defaults to: now()
	pub updated:Option<NaiveDateTime>,
	pub updatedby:Option<f64>,
	pub valutadate:Option<NaiveDateTime>,
	/// has one
	pub c_bankstatement_id_c_bankstatement:Option<CBankstatement>,
	/// has one
	pub c_bankaccount_id_c_bankaccount:Option<CBankaccount>,
	/// has one
	pub c_payment_id_c_payment:Option<CPayment>,
	/// has one
	pub c_currency_id_c_currency:Option<CCurrency>,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub c_invoice_id_c_invoice:Option<CInvoice>,
	/// has one
	pub c_charge_id_c_charge:Option<CCharge>,
	/// has one
	pub c_bankstatementline_id_c_bankstatementline:Option<CBankstatementline>,
}

pub struct IBpartner {
	/// primary
	/// not nullable 
	pub i_bpartner_id:f64,
	pub ad_client_id:Option<f64>,
	pub ad_org_id:Option<f64>,
	pub ad_user_id:Option<f64>,
	pub address1:Option<String>,
	pub address2:Option<String>,
	pub birthday:Option<NaiveDateTime>,
	pub bpcontactgreeting:Option<String>,
	pub c_bp_group_id:Option<f64>,
	pub c_bpartner_id:Option<f64>,
	pub c_bpartner_location_id:Option<f64>,
	pub c_country_id:Option<f64>,
	pub c_greeting_id:Option<f64>,
	pub c_region_id:Option<f64>,
	pub city:Option<String>,
	pub comments:Option<String>,
	pub contactdescription:Option<String>,
	pub contactname:Option<String>,
	pub countrycode:Option<String>,
	/// defaults to: now()
	pub created:Option<NaiveDateTime>,
	pub createdby:Option<f64>,
	pub description:Option<String>,
	pub duns:Option<String>,
	pub email:Option<String>,
	pub fax:Option<String>,
	pub groupvalue:Option<String>,
	pub i_errormsg:Option<String>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub i_isimported:String,
	pub interestareaname:Option<String>,
	/// defaults to: 'Y'::bpchar
	pub isactive:Option<String>,
	pub naics:Option<String>,
	pub name:Option<String>,
	pub name2:Option<String>,
	pub password:Option<String>,
	pub phone:Option<String>,
	pub phone2:Option<String>,
	pub postal:Option<String>,
	pub postal_add:Option<String>,
	/// defaults to: 'N'::bpchar
	pub processed:Option<String>,
	pub processing:Option<String>,
	pub r_interestarea_id:Option<f64>,
	pub regionname:Option<String>,
	pub taxid:Option<String>,
	pub title:Option<String>,
	/// defaults to: now()
	pub updated:Option<NaiveDateTime>,
	pub updatedby:Option<f64>,
	pub value:Option<String>,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub c_bp_group_id_c_bp_group:Option<CBpGroup>,
	/// has one
	pub c_bpartner_location_id_c_bpartner_location:Option<CBpartnerLocation>,
	/// has one
	pub c_region_id_c_region:Option<CRegion>,
	/// has one
	pub c_country_id_c_country:Option<CCountry>,
	/// has one
	pub c_greeting_id_c_greeting:Option<CGreeting>,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
	/// has one
	pub r_interestarea_id_r_interestarea:Option<RInterestarea>,
}

pub struct IConversionRate {
	/// primary
	/// not nullable 
	pub i_conversion_rate_id:f64,
	pub ad_client_id:Option<f64>,
	pub ad_org_id:Option<f64>,
	pub c_conversion_rate_id:Option<f64>,
	pub c_conversiontype_id:Option<f64>,
	pub c_currency_id:Option<f64>,
	pub c_currency_id_to:Option<f64>,
	pub conversiontypevalue:Option<String>,
	/// defaults to: now()
	pub created:Option<NaiveDateTime>,
	pub createdby:Option<f64>,
	/// defaults to: 'N'::bpchar
	pub createreciprocalrate:Option<String>,
	pub dividerate:Option<f64>,
	pub i_errormsg:Option<String>,
	/// defaults to: 'N'::bpchar
	pub i_isimported:Option<String>,
	/// defaults to: 'Y'::bpchar
	pub isactive:Option<String>,
	pub iso_code:Option<String>,
	pub iso_code_to:Option<String>,
	pub multiplyrate:Option<f64>,
	/// defaults to: 'N'::bpchar
	pub processed:Option<String>,
	pub processing:Option<String>,
	/// defaults to: now()
	pub updated:Option<NaiveDateTime>,
	pub updatedby:Option<f64>,
	pub validfrom:Option<NaiveDateTime>,
	pub validto:Option<NaiveDateTime>,
	/// has one
	pub c_conversion_rate_id_c_conversion_rate:Option<CConversionRate>,
	/// has one
	pub c_currency_id_c_currency:Option<CCurrency>,
	/// has one
	pub c_currency_id_to_c_currency:Option<CCurrency>,
	/// has one
	pub c_conversiontype_id_c_conversiontype:Option<CConversiontype>,
}

pub struct IElementvalue {
	/// primary
	/// not nullable 
	pub i_elementvalue_id:f64,
	pub accountsign:Option<String>,
	pub accounttype:Option<String>,
	pub ad_client_id:Option<f64>,
	pub ad_column_id:Option<f64>,
	pub ad_org_id:Option<f64>,
	pub c_element_id:Option<f64>,
	pub c_elementvalue_id:Option<f64>,
	/// defaults to: now()
	pub created:Option<NaiveDateTime>,
	pub createdby:Option<f64>,
	pub default_account:Option<String>,
	pub description:Option<String>,
	pub elementname:Option<String>,
	pub i_errormsg:Option<String>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub i_isimported:String,
	/// defaults to: 'Y'::bpchar
	pub isactive:Option<String>,
	/// defaults to: 'N'::bpchar
	pub isdoccontrolled:Option<String>,
	/// defaults to: 'N'::bpchar
	pub issummary:Option<String>,
	pub name:Option<String>,
	pub parentelementvalue_id:Option<f64>,
	pub parentvalue:Option<String>,
	/// defaults to: 'Y'::bpchar
	pub postactual:Option<String>,
	/// defaults to: 'Y'::bpchar
	pub postbudget:Option<String>,
	/// defaults to: 'Y'::bpchar
	pub postencumbrance:Option<String>,
	/// defaults to: 'Y'::bpchar
	pub poststatistical:Option<String>,
	/// defaults to: 'N'::bpchar
	pub processed:Option<String>,
	pub processing:Option<String>,
	/// defaults to: now()
	pub updated:Option<NaiveDateTime>,
	pub updatedby:Option<f64>,
	pub value:Option<String>,
	/// has one
	pub c_element_id_c_element:Option<CElement>,
	/// has one
	pub c_elementvalue_id_c_elementvalue:Option<CElementvalue>,
	/// has one
	pub parentelementvalue_id_c_elementvalue:Option<CElementvalue>,
	/// has one
	pub ad_column_id_ad_column:Option<AdColumn>,
}

pub struct IFajournal {
	/// primary
	/// not nullable 
	pub i_fajournal_id:f64,
	pub a_asset_id:Option<f64>,
	pub a_entry_type:Option<String>,
	pub account_id:Option<f64>,
	pub accountvalue:Option<String>,
	pub acctschemaname:Option<String>,
	pub ad_client_id:Option<f64>,
	pub ad_org_id:Option<f64>,
	pub ad_orgdoc_id:Option<f64>,
	pub ad_orgtrx_id:Option<f64>,
	pub amtacctcr:Option<f64>,
	pub amtacctdr:Option<f64>,
	pub amtsourcecr:Option<f64>,
	pub amtsourcedr:Option<f64>,
	pub batchdescription:Option<String>,
	pub batchdocumentno:Option<String>,
	pub bpartnervalue:Option<String>,
	pub c_acctschema_id:Option<f64>,
	pub c_activity_id:Option<f64>,
	pub c_bpartner_id:Option<f64>,
	pub c_campaign_id:Option<f64>,
	pub c_conversiontype_id:Option<f64>,
	pub c_currency_id:Option<f64>,
	pub c_doctype_id:Option<f64>,
	pub c_locfrom_id:Option<f64>,
	pub c_locto_id:Option<f64>,
	pub c_period_id:Option<f64>,
	pub c_project_id:Option<f64>,
	pub c_salesregion_id:Option<f64>,
	pub c_subacct_id:Option<f64>,
	pub c_uom_id:Option<f64>,
	pub c_validcombination_id:Option<f64>,
	pub categoryname:Option<String>,
	pub clientvalue:Option<String>,
	pub conversiontypevalue:Option<String>,
	pub created:Option<NaiveDateTime>,
	pub createdby:Option<f64>,
	pub currencyrate:Option<f64>,
	pub currencyratetype:Option<String>,
	pub dateacct:Option<NaiveDateTime>,
	pub description:Option<String>,
	pub doctypename:Option<String>,
	pub gl_budget_id:Option<f64>,
	pub gl_category_id:Option<f64>,
	pub gl_journal_id:Option<f64>,
	pub gl_journalbatch_id:Option<f64>,
	pub gl_journalline_id:Option<f64>,
	pub i_errormsg:Option<String>,
	/// not nullable 
	pub i_isimported:String,
	/// not nullable 
	pub isactive:String,
	pub isdepreciated:Option<String>,
	pub iso_code:Option<String>,
	pub journaldocumentno:Option<String>,
	pub line:Option<f64>,
	pub m_product_id:Option<f64>,
	pub orgtrxvalue:Option<String>,
	pub orgvalue:Option<String>,
	pub postingtype:Option<String>,
	pub processed:Option<String>,
	pub processing:Option<String>,
	pub productvalue:Option<String>,
	pub projectvalue:Option<String>,
	pub qty:Option<f64>,
	pub sku:Option<String>,
	pub upc:Option<String>,
	pub updated:Option<NaiveDateTime>,
	pub updatedby:Option<f64>,
	pub user1_id:Option<f64>,
	pub user2_id:Option<f64>,
	pub userelement1_id:Option<f64>,
	pub userelement2_id:Option<f64>,
	/// has one
	pub ad_orgdoc_id_ad_org:Option<AdOrg>,
	/// has one
	pub ad_orgtrx_id_ad_org:Option<AdOrg>,
	/// has one
	pub account_id_c_elementvalue:Option<CElementvalue>,
	/// has one
	pub c_acctschema_id_c_acctschema:Option<CAcctschema>,
	/// has one
	pub gl_budget_id_gl_budget:Option<GlBudget>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub user2_id_c_elementvalue:Option<CElementvalue>,
	/// has one
	pub user1_id_c_elementvalue:Option<CElementvalue>,
	/// has one
	pub gl_journal_id_gl_journal:Option<GlJournal>,
	/// has one
	pub gl_journalline_id_gl_journalline:Option<GlJournalline>,
	/// has one
	pub gl_journalbatch_id_gl_journalbatch:Option<GlJournalbatch>,
	/// has one
	pub gl_category_id_gl_category:Option<GlCategory>,
	/// has one
	pub c_validcombination_id_c_validcombination:Option<CValidcombination>,
	/// has one
	pub c_uom_id_c_uom:Option<CUom>,
	/// has one
	pub c_activity_id_c_activity:Option<CActivity>,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub c_campaign_id_c_campaign:Option<CCampaign>,
	/// has one
	pub c_currency_id_c_currency:Option<CCurrency>,
	/// has one
	pub c_doctype_id_c_doctype:Option<CDoctype>,
	/// has one
	pub c_period_id_c_period:Option<CPeriod>,
	/// has one
	pub c_project_id_c_project:Option<CProject>,
	/// has one
	pub c_salesregion_id_c_salesregion:Option<CSalesregion>,
}

pub struct IGljournal {
	/// primary
	/// not nullable 
	pub i_gljournal_id:f64,
	pub account_id:Option<f64>,
	pub accountvalue:Option<String>,
	pub acctschemaname:Option<String>,
	pub ad_client_id:Option<f64>,
	pub ad_org_id:Option<f64>,
	pub ad_orgdoc_id:Option<f64>,
	pub ad_orgtrx_id:Option<f64>,
	/// defaults to: 0
	pub amtacctcr:Option<f64>,
	/// defaults to: 0
	pub amtacctdr:Option<f64>,
	/// defaults to: 0
	pub amtsourcecr:Option<f64>,
	/// defaults to: 0
	pub amtsourcedr:Option<f64>,
	pub batchdescription:Option<String>,
	pub batchdocumentno:Option<String>,
	pub bpartnervalue:Option<String>,
	pub c_acctschema_id:Option<f64>,
	pub c_activity_id:Option<f64>,
	pub c_bpartner_id:Option<f64>,
	pub c_campaign_id:Option<f64>,
	pub c_conversiontype_id:Option<f64>,
	pub c_currency_id:Option<f64>,
	pub c_doctype_id:Option<f64>,
	pub c_locfrom_id:Option<f64>,
	pub c_locto_id:Option<f64>,
	pub c_period_id:Option<f64>,
	pub c_project_id:Option<f64>,
	pub c_salesregion_id:Option<f64>,
	pub c_uom_id:Option<f64>,
	pub c_validcombination_id:Option<f64>,
	pub categoryname:Option<String>,
	pub clientvalue:Option<String>,
	pub conversiontypevalue:Option<String>,
	/// defaults to: now()
	pub created:Option<NaiveDateTime>,
	pub createdby:Option<f64>,
	/// defaults to: 0
	pub currencyrate:Option<f64>,
	pub dateacct:Option<NaiveDateTime>,
	pub description:Option<String>,
	pub doctypename:Option<String>,
	pub gl_budget_id:Option<f64>,
	pub gl_category_id:Option<f64>,
	pub gl_journal_id:Option<f64>,
	pub gl_journalbatch_id:Option<f64>,
	pub gl_journalline_id:Option<f64>,
	pub i_errormsg:Option<String>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub i_isimported:String,
	/// defaults to: 'Y'::bpchar
	pub isactive:Option<String>,
	pub iscreatenewbatch:Option<String>,
	pub iscreatenewjournal:Option<String>,
	pub iso_code:Option<String>,
	pub journaldocumentno:Option<String>,
	pub line:Option<f64>,
	pub m_product_id:Option<f64>,
	pub orgtrxvalue:Option<String>,
	pub orgvalue:Option<String>,
	pub postingtype:Option<String>,
	/// defaults to: 'N'::bpchar
	pub processed:Option<String>,
	pub processing:Option<String>,
	pub productvalue:Option<String>,
	pub projectvalue:Option<String>,
	/// defaults to: 0
	pub qty:Option<f64>,
	pub sku:Option<String>,
	pub upc:Option<String>,
	/// defaults to: now()
	pub updated:Option<NaiveDateTime>,
	pub updatedby:Option<f64>,
	pub user1_id:Option<f64>,
	pub user2_id:Option<f64>,
	/// has one
	pub ad_orgdoc_id_ad_org:Option<AdOrg>,
	/// has one
	pub gl_journalbatch_id_gl_journalbatch:Option<GlJournalbatch>,
	/// has one
	pub gl_journal_id_gl_journal:Option<GlJournal>,
	/// has one
	pub c_acctschema_id_c_acctschema:Option<CAcctschema>,
	/// has one
	pub c_doctype_id_c_doctype:Option<CDoctype>,
	/// has one
	pub gl_category_id_gl_category:Option<GlCategory>,
	/// has one
	pub c_period_id_c_period:Option<CPeriod>,
	/// has one
	pub gl_budget_id_gl_budget:Option<GlBudget>,
	/// has one
	pub gl_journalline_id_gl_journalline:Option<GlJournalline>,
	/// has one
	pub c_currency_id_c_currency:Option<CCurrency>,
	/// has one
	pub c_conversiontype_id_c_conversiontype:Option<CConversiontype>,
	/// has one
	pub c_uom_id_c_uom:Option<CUom>,
	/// has one
	pub c_validcombination_id_c_validcombination:Option<CValidcombination>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub account_id_c_elementvalue:Option<CElementvalue>,
	/// has one
	pub ad_orgtrx_id_ad_org:Option<AdOrg>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub c_project_id_c_project:Option<CProject>,
	/// has one
	pub c_locto_id_c_location:Option<CLocation>,
	/// has one
	pub c_locfrom_id_c_location:Option<CLocation>,
	/// has one
	pub c_salesregion_id_c_salesregion:Option<CSalesregion>,
	/// has one
	pub c_activity_id_c_activity:Option<CActivity>,
	/// has one
	pub c_campaign_id_c_campaign:Option<CCampaign>,
	/// has one
	pub user1_id_c_elementvalue:Option<CElementvalue>,
	/// has one
	pub user2_id_c_elementvalue:Option<CElementvalue>,
}

pub struct IInoutlineconfirm {
	/// primary
	/// not nullable 
	pub i_inoutlineconfirm_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub confirmationno:Option<String>,
	/// defaults to: 0
	pub confirmedqty:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 0
	pub differenceqty:Option<f64>,
	pub i_errormsg:Option<String>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub i_isimported:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub m_inoutlineconfirm_id:Option<f64>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	pub processing:Option<String>,
	/// defaults to: 0
	pub scrappedqty:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub m_inoutlineconfirm_id_m_inoutlineconfirm:Option<MInoutlineconfirm>,
}

pub struct IInventory {
	/// primary
	/// not nullable 
	pub i_inventory_id:f64,
	pub ad_client_id:Option<f64>,
	pub ad_org_id:Option<f64>,
	/// defaults to: now()
	pub created:Option<NaiveDateTime>,
	pub createdby:Option<f64>,
	pub description:Option<String>,
	pub i_errormsg:Option<String>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub i_isimported:String,
	/// defaults to: 'Y'::bpchar
	pub isactive:Option<String>,
	pub locatorvalue:Option<String>,
	pub lot:Option<String>,
	pub m_inventory_id:Option<f64>,
	pub m_inventoryline_id:Option<f64>,
	pub m_locator_id:Option<f64>,
	pub m_product_id:Option<f64>,
	pub m_warehouse_id:Option<f64>,
	pub movementdate:Option<NaiveDateTime>,
	/// defaults to: 'N'::bpchar
	pub processed:Option<String>,
	pub processing:Option<String>,
	/// defaults to: 0
	pub qtybook:Option<f64>,
	/// defaults to: 0
	pub qtycount:Option<f64>,
	pub serno:Option<String>,
	pub upc:Option<String>,
	/// defaults to: now()
	pub updated:Option<NaiveDateTime>,
	pub updatedby:Option<f64>,
	pub value:Option<String>,
	pub warehousevalue:Option<String>,
	pub x:Option<String>,
	pub y:Option<String>,
	pub z:Option<String>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub m_locator_id_m_locator:Option<MLocator>,
	/// has one
	pub m_warehouse_id_m_warehouse:Option<MWarehouse>,
	/// has one
	pub m_inventory_id_m_inventory:Option<MInventory>,
	/// has one
	pub m_inventoryline_id_m_inventoryline:Option<MInventoryline>,
}

pub struct IInvoice {
	/// primary
	/// not nullable 
	pub i_invoice_id:f64,
	pub activityvalue:Option<String>,
	pub ad_client_id:Option<f64>,
	pub ad_org_id:Option<f64>,
	pub ad_orgtrx_id:Option<f64>,
	pub ad_user_id:Option<f64>,
	pub address1:Option<String>,
	pub address2:Option<String>,
	pub bpartnervalue:Option<String>,
	pub c_activity_id:Option<f64>,
	pub c_bpartner_id:Option<f64>,
	pub c_bpartner_location_id:Option<f64>,
	pub c_campaign_id:Option<f64>,
	pub c_charge_id:Option<f64>,
	pub c_country_id:Option<f64>,
	pub c_currency_id:Option<f64>,
	pub c_doctype_id:Option<f64>,
	pub c_invoice_id:Option<f64>,
	pub c_invoiceline_id:Option<f64>,
	pub c_location_id:Option<f64>,
	pub c_paymentterm_id:Option<f64>,
	pub c_project_id:Option<f64>,
	pub c_region_id:Option<f64>,
	pub c_tax_id:Option<f64>,
	pub chargename:Option<String>,
	pub city:Option<String>,
	pub contactname:Option<String>,
	pub countrycode:Option<String>,
	/// defaults to: now()
	pub created:Option<NaiveDateTime>,
	pub createdby:Option<f64>,
	pub dateacct:Option<NaiveDateTime>,
	pub dateinvoiced:Option<NaiveDateTime>,
	pub description:Option<String>,
	pub doctypename:Option<String>,
	pub documentno:Option<String>,
	pub email:Option<String>,
	pub i_errormsg:Option<String>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub i_isimported:String,
	/// defaults to: 'Y'::bpchar
	pub isactive:Option<String>,
	/// defaults to: 'Y'::bpchar
	pub issotrx:Option<String>,
	pub linedescription:Option<String>,
	pub m_pricelist_id:Option<f64>,
	pub m_product_id:Option<f64>,
	pub name:Option<String>,
	pub paymenttermvalue:Option<String>,
	pub phone:Option<String>,
	pub postal:Option<String>,
	/// defaults to: 0
	pub priceactual:Option<f64>,
	/// defaults to: 'N'::bpchar
	pub processed:Option<String>,
	pub processing:Option<String>,
	pub productvalue:Option<String>,
	pub projectvalue:Option<String>,
	/// defaults to: 0
	pub qtyordered:Option<f64>,
	pub regionname:Option<String>,
	pub salesrep_id:Option<f64>,
	pub sku:Option<String>,
	/// defaults to: 0
	pub taxamt:Option<f64>,
	pub taxindicator:Option<String>,
	pub upc:Option<String>,
	/// defaults to: now()
	pub updated:Option<NaiveDateTime>,
	pub updatedby:Option<f64>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub ad_orgtrx_id_ad_org:Option<AdOrg>,
	/// has one
	pub m_pricelist_id_m_pricelist:Option<MPricelist>,
	/// has one
	pub c_currency_id_c_currency:Option<CCurrency>,
	/// has one
	pub salesrep_id_ad_user:Option<AdUser>,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub c_bpartner_location_id_c_bpartner_location:Option<CBpartnerLocation>,
	/// has one
	pub c_location_id_c_location:Option<CLocation>,
	/// has one
	pub c_region_id_c_region:Option<CRegion>,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
	/// has one
	pub c_country_id_c_country:Option<CCountry>,
	/// has one
	pub c_doctype_id_c_doctype:Option<CDoctype>,
	/// has one
	pub c_paymentterm_id_c_paymentterm:Option<CPaymentterm>,
	/// has one
	pub c_project_id_c_project:Option<CProject>,
	/// has one
	pub c_campaign_id_c_campaign:Option<CCampaign>,
	/// has one
	pub c_activity_id_c_activity:Option<CActivity>,
	/// has one
	pub c_invoice_id_c_invoice:Option<CInvoice>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub c_tax_id_c_tax:Option<CTax>,
	/// has one
	pub c_invoiceline_id_c_invoiceline:Option<CInvoiceline>,
	/// has one
	pub c_charge_id_c_charge:Option<CCharge>,
}

pub struct IOrder {
	/// primary
	/// not nullable 
	pub i_order_id:f64,
	pub ad_client_id:Option<f64>,
	pub ad_org_id:Option<f64>,
	pub ad_orgtrx_id:Option<f64>,
	pub ad_user_id:Option<f64>,
	pub address1:Option<String>,
	pub address2:Option<String>,
	pub billto_id:Option<f64>,
	pub bpartnervalue:Option<String>,
	pub c_activity_id:Option<f64>,
	pub c_bpartner_id:Option<f64>,
	pub c_bpartner_location_id:Option<f64>,
	pub c_campaign_id:Option<f64>,
	pub c_charge_id:Option<f64>,
	pub c_country_id:Option<f64>,
	pub c_currency_id:Option<f64>,
	pub c_doctype_id:Option<f64>,
	pub c_location_id:Option<f64>,
	pub c_order_id:Option<f64>,
	pub c_orderline_id:Option<f64>,
	pub c_paymentterm_id:Option<f64>,
	pub c_project_id:Option<f64>,
	pub c_region_id:Option<f64>,
	pub c_tax_id:Option<f64>,
	pub c_uom_id:Option<f64>,
	pub chargename:Option<String>,
	pub city:Option<String>,
	pub contactname:Option<String>,
	pub countrycode:Option<String>,
	/// defaults to: now()
	pub created:Option<NaiveDateTime>,
	pub createdby:Option<f64>,
	pub dateacct:Option<NaiveDateTime>,
	pub dateordered:Option<NaiveDateTime>,
	pub description:Option<String>,
	pub doctypename:Option<String>,
	pub documentno:Option<String>,
	pub email:Option<String>,
	/// defaults to: 0
	pub freightamt:Option<f64>,
	pub i_errormsg:Option<String>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub i_isimported:String,
	/// defaults to: 'Y'::bpchar
	pub isactive:Option<String>,
	/// defaults to: 'Y'::bpchar
	pub issotrx:Option<String>,
	pub linedescription:Option<String>,
	pub m_pricelist_id:Option<f64>,
	pub m_product_id:Option<f64>,
	pub m_shipper_id:Option<f64>,
	pub m_warehouse_id:Option<f64>,
	pub name:Option<String>,
	pub paymenttermvalue:Option<String>,
	pub phone:Option<String>,
	pub postal:Option<String>,
	/// defaults to: 0
	pub priceactual:Option<f64>,
	/// defaults to: 'N'::bpchar
	pub processed:Option<String>,
	pub processing:Option<String>,
	pub productvalue:Option<String>,
	/// defaults to: 0
	pub qtyordered:Option<f64>,
	pub regionname:Option<String>,
	pub salesrep_id:Option<f64>,
	pub sku:Option<String>,
	/// defaults to: 0
	pub taxamt:Option<f64>,
	pub taxindicator:Option<String>,
	pub upc:Option<String>,
	/// defaults to: now()
	pub updated:Option<NaiveDateTime>,
	pub updatedby:Option<f64>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub ad_orgtrx_id_ad_org:Option<AdOrg>,
	/// has one
	pub salesrep_id_ad_user:Option<AdUser>,
	/// has one
	pub m_warehouse_id_m_warehouse:Option<MWarehouse>,
	/// has one
	pub m_pricelist_id_m_pricelist:Option<MPricelist>,
	/// has one
	pub c_currency_id_c_currency:Option<CCurrency>,
	/// has one
	pub m_shipper_id_m_shipper:Option<MShipper>,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub c_bpartner_location_id_c_bpartner_location:Option<CBpartnerLocation>,
	/// has one
	pub billto_id_c_bpartner_location:Option<CBpartnerLocation>,
	/// has one
	pub c_location_id_c_location:Option<CLocation>,
	/// has one
	pub c_region_id_c_region:Option<CRegion>,
	/// has one
	pub c_country_id_c_country:Option<CCountry>,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
	/// has one
	pub c_project_id_c_project:Option<CProject>,
	/// has one
	pub c_activity_id_c_activity:Option<CActivity>,
	/// has one
	pub c_doctype_id_c_doctype:Option<CDoctype>,
	/// has one
	pub c_paymentterm_id_c_paymentterm:Option<CPaymentterm>,
	/// has one
	pub c_order_id_c_order:Option<COrder>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub c_tax_id_c_tax:Option<CTax>,
	/// has one
	pub c_orderline_id_c_orderline:Option<COrderline>,
	/// has one
	pub c_uom_id_c_uom:Option<CUom>,
	/// has one
	pub c_campaign_id_c_campaign:Option<CCampaign>,
}

pub struct IPayment {
	/// primary
	/// not nullable 
	pub i_payment_id:f64,
	pub a_city:Option<String>,
	pub a_country:Option<String>,
	pub a_email:Option<String>,
	pub a_ident_dl:Option<String>,
	pub a_ident_ssn:Option<String>,
	pub a_name:Option<String>,
	pub a_state:Option<String>,
	pub a_street:Option<String>,
	pub a_zip:Option<String>,
	pub accountno:Option<String>,
	pub ad_client_id:Option<f64>,
	pub ad_org_id:Option<f64>,
	pub bankaccountno:Option<String>,
	pub bpartnervalue:Option<String>,
	pub c_bankaccount_id:Option<f64>,
	pub c_bpartner_id:Option<f64>,
	pub c_charge_id:Option<f64>,
	pub c_currency_id:Option<f64>,
	pub c_doctype_id:Option<f64>,
	pub c_invoice_id:Option<f64>,
	pub c_payment_id:Option<f64>,
	/// defaults to: 0
	pub chargeamt:Option<f64>,
	pub chargename:Option<String>,
	pub checkno:Option<String>,
	/// defaults to: now()
	pub created:Option<NaiveDateTime>,
	pub createdby:Option<f64>,
	pub creditcardexpmm:Option<f64>,
	pub creditcardexpyy:Option<f64>,
	pub creditcardnumber:Option<String>,
	pub creditcardtype:Option<String>,
	pub creditcardvv:Option<String>,
	pub dateacct:Option<NaiveDateTime>,
	pub datetrx:Option<NaiveDateTime>,
	/// defaults to: 0
	pub discountamt:Option<f64>,
	pub doctypename:Option<String>,
	pub documentno:Option<String>,
	pub i_errormsg:Option<String>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub i_isimported:String,
	pub invoicedocumentno:Option<String>,
	/// defaults to: 'Y'::bpchar
	pub isactive:Option<String>,
	/// defaults to: 'N'::bpchar
	pub isapproved:Option<String>,
	/// defaults to: 'N'::bpchar
	pub isdelayedcapture:Option<String>,
	pub iso_code:Option<String>,
	/// defaults to: 'N'::bpchar
	pub isoverunderpayment:Option<String>,
	/// defaults to: 'Y'::bpchar
	pub isreceipt:Option<String>,
	/// defaults to: 'N'::bpchar
	pub isselfservice:Option<String>,
	pub micr:Option<String>,
	pub orig_trxid:Option<String>,
	/// defaults to: 0
	pub overunderamt:Option<f64>,
	/// defaults to: 0
	pub payamt:Option<f64>,
	pub ponum:Option<String>,
	/// defaults to: 'N'::bpchar
	pub processed:Option<String>,
	pub processing:Option<String>,
	pub r_authcode:Option<String>,
	pub r_info:Option<String>,
	pub r_pnref:Option<String>,
	pub r_respmsg:Option<String>,
	pub r_result:Option<String>,
	pub routingno:Option<String>,
	pub swipe:Option<String>,
	/// defaults to: 0
	pub taxamt:Option<f64>,
	pub tendertype:Option<String>,
	pub trxtype:Option<String>,
	/// defaults to: now()
	pub updated:Option<NaiveDateTime>,
	pub updatedby:Option<f64>,
	pub voiceauthcode:Option<String>,
	/// defaults to: 0
	pub writeoffamt:Option<f64>,
	/// has one
	pub c_payment_id_c_payment:Option<CPayment>,
	/// has one
	pub c_doctype_id_c_doctype:Option<CDoctype>,
	/// has one
	pub c_bankaccount_id_c_bankaccount:Option<CBankaccount>,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub c_invoice_id_c_invoice:Option<CInvoice>,
	/// has one
	pub c_currency_id_c_currency:Option<CCurrency>,
	/// has one
	pub c_charge_id_c_charge:Option<CCharge>,
}

pub struct IPricelist {
	/// primary
	/// not nullable 
	pub i_pricelist_id:f64,
	/// defaults to: NULL::numeric
	pub ad_client_id:Option<f64>,
	/// defaults to: NULL::numeric
	pub ad_org_id:Option<f64>,
	pub bpartner_value:Option<String>,
	pub breakvalue:Option<f64>,
	pub c_bpartner_id:Option<f64>,
	pub c_currency_id:Option<f64>,
	pub c_uom_id:Option<f64>,
	pub created:Option<NaiveDateTime>,
	pub createdby:Option<f64>,
	pub description:Option<String>,
	pub enforcepricelimit:Option<String>,
	pub i_errormsg:Option<String>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub i_isimported:String,
	/// defaults to: 'Y'::bpchar
	pub isactive:Option<String>,
	pub iso_code:Option<String>,
	pub issopricelist:Option<String>,
	pub istaxincluded:Option<String>,
	pub m_pricelist_id:Option<f64>,
	pub m_pricelist_version_id:Option<f64>,
	pub m_product_id:Option<f64>,
	pub name:Option<String>,
	pub pricelimit:Option<f64>,
	pub pricelist:Option<f64>,
	pub priceprecision:Option<f64>,
	pub pricestd:Option<f64>,
	/// defaults to: 'N'::bpchar
	pub processed:Option<String>,
	pub processing:Option<String>,
	pub productvalue:Option<String>,
	pub updated:Option<NaiveDateTime>,
	pub updatedby:Option<f64>,
	pub validfrom:Option<NaiveDateTime>,
	pub x12de355:Option<String>,
}

pub struct IProduct {
	/// primary
	/// not nullable 
	pub i_product_id:f64,
	pub ad_client_id:Option<f64>,
	pub ad_org_id:Option<f64>,
	pub bpartner_value:Option<String>,
	pub c_bpartner_id:Option<f64>,
	pub c_currency_id:Option<f64>,
	pub c_uom_id:Option<f64>,
	pub classification:Option<String>,
	/// defaults to: 0
	pub costperorder:Option<f64>,
	/// defaults to: now()
	pub created:Option<NaiveDateTime>,
	pub createdby:Option<f64>,
	pub deliverytime_promised:Option<f64>,
	pub description:Option<String>,
	pub descriptionurl:Option<String>,
	/// defaults to: 'N'::bpchar
	pub discontinued:Option<String>,
	pub discontinuedby:Option<NaiveDateTime>,
	pub documentnote:Option<String>,
	pub help:Option<String>,
	pub i_errormsg:Option<String>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub i_isimported:String,
	pub imageurl:Option<String>,
	/// defaults to: 'Y'::bpchar
	pub isactive:Option<String>,
	pub iso_code:Option<String>,
	pub m_product_category_id:Option<f64>,
	pub m_product_id:Option<f64>,
	pub manufacturer:Option<String>,
	pub name:Option<String>,
	/// defaults to: 0
	pub order_min:Option<f64>,
	/// defaults to: 0
	pub order_pack:Option<f64>,
	pub priceeffective:Option<NaiveDateTime>,
	pub pricelimit:Option<f64>,
	/// defaults to: 0
	pub pricelist:Option<f64>,
	/// defaults to: 0
	pub pricepo:Option<f64>,
	pub pricestd:Option<f64>,
	/// defaults to: 'N'::bpchar
	pub processed:Option<String>,
	pub processing:Option<String>,
	pub productcategory_value:Option<String>,
	/// defaults to: 'I'::bpchar
	pub producttype:Option<String>,
	/// defaults to: 0
	pub royaltyamt:Option<f64>,
	pub shelfdepth:Option<f64>,
	pub shelfheight:Option<f64>,
	pub shelfwidth:Option<f64>,
	pub sku:Option<String>,
	pub unitsperpallet:Option<f64>,
	pub upc:Option<String>,
	/// defaults to: now()
	pub updated:Option<NaiveDateTime>,
	pub updatedby:Option<f64>,
	pub value:Option<String>,
	pub vendorcategory:Option<String>,
	pub vendorproductno:Option<String>,
	/// defaults to: 0
	pub volume:Option<f64>,
	/// defaults to: 0
	pub weight:Option<f64>,
	pub x12de355:Option<String>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub c_uom_id_c_uom:Option<CUom>,
	/// has one
	pub m_product_category_id_m_product_category:Option<MProductCategory>,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub c_currency_id_c_currency:Option<CCurrency>,
}

pub struct IReportline {
	/// primary
	/// not nullable 
	pub i_reportline_id:f64,
	pub ad_client_id:Option<f64>,
	pub ad_org_id:Option<f64>,
	pub amounttype:Option<String>,
	pub c_elementvalue_id:Option<f64>,
	pub calculationtype:Option<String>,
	/// defaults to: now()
	pub created:Option<NaiveDateTime>,
	pub createdby:Option<f64>,
	pub description:Option<String>,
	pub elementvalue:Option<String>,
	pub i_errormsg:Option<String>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub i_isimported:String,
	/// defaults to: 'Y'::bpchar
	pub isactive:Option<String>,
	/// defaults to: 'Y'::bpchar
	pub isprinted:Option<String>,
	/// defaults to: 'N'::bpchar
	pub issummary:Option<String>,
	pub linetype:Option<String>,
	pub name:Option<String>,
	pub pa_reportline_id:Option<f64>,
	pub pa_reportlineset_id:Option<f64>,
	pub pa_reportsource_id:Option<f64>,
	pub postingtype:Option<String>,
	/// defaults to: 'N'::bpchar
	pub processed:Option<String>,
	pub processing:Option<String>,
	pub reportlinesetname:Option<String>,
	pub seqno:Option<f64>,
	/// defaults to: now()
	pub updated:Option<NaiveDateTime>,
	pub updatedby:Option<f64>,
	/// has one
	pub pa_reportlineset_id_pa_reportlineset:Option<PaReportlineset>,
	/// has one
	pub pa_reportline_id_pa_reportline:Option<PaReportline>,
	/// has one
	pub pa_reportsource_id_pa_reportsource:Option<PaReportsource>,
	/// has one
	pub c_elementvalue_id_c_elementvalue:Option<CElementvalue>,
}

pub struct ImpProcessor {
	/// primary
	/// not nullable 
	pub imp_processor_id:f64,
	pub account:Option<String>,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub datelastrun:Option<NaiveDateTime>,
	pub datenextrun:Option<NaiveDateTime>,
	pub description:Option<String>,
	/// not nullable 
	pub frequency:f64,
	/// not nullable 
	pub frequencytype:String,
	pub help:Option<String>,
	pub host:Option<String>,
	/// not nullable 
	pub imp_processor_type_id:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: (7)::numeric
	/// not nullable 
	pub keeplogdays:f64,
	/// not nullable 
	pub name:String,
	pub passwordinfo:Option<String>,
	pub port:Option<f64>,
	pub processing:Option<String>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
	/// has one
	pub imp_processor_type_id_imp_processor_type:Option<ImpProcessorType>,
	/// has many
	pub imp_processorlog:Option<Vec<ImpProcessorlog>>,
	/// has many
	pub imp_processorparameter:Option<Vec<ImpProcessorparameter>>,
}

pub struct ImpProcessorType {
	/// primary
	/// not nullable 
	pub imp_processor_type_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub javaclass:String,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
	/// has many
	pub imp_processor:Option<Vec<ImpProcessor>>,
}

pub struct ImpProcessorlog {
	/// primary
	/// not nullable 
	pub imp_processorlog_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub binarydata:Option<Vec<u8>>,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// not nullable 
	pub imp_processor_id:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub iserror:String,
	pub reference:Option<String>,
	pub summary:Option<String>,
	pub textmsg:Option<String>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub imp_processor_id_imp_processor:Option<ImpProcessor>,
}

pub struct ImpProcessorparameter {
	/// primary
	/// not nullable 
	pub imp_processorparameter_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// not nullable 
	pub imp_processor_id:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	pub parametervalue:Option<String>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
	/// has one
	pub imp_processor_id_imp_processor:Option<ImpProcessor>,
}

pub struct KCategory {
	/// primary
	/// not nullable 
	pub k_category_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has many
	pub k_categoryvalue:Option<Vec<KCategoryvalue>>,
	/// has many
	pub k_entrycategory:Option<Vec<KEntrycategory>>,
}

pub struct KCategoryvalue {
	/// primary
	/// not nullable 
	pub k_categoryvalue_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub k_category_id:f64,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub k_category_id_k_category:Option<KCategory>,
	/// has many
	pub k_entrycategory:Option<Vec<KEntrycategory>>,
}

pub struct KComment {
	/// primary
	/// not nullable 
	pub k_comment_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_session_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub ispublic:String,
	/// not nullable 
	pub k_entry_id:f64,
	/// not nullable 
	pub rating:f64,
	/// not nullable 
	pub textmsg:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub k_entry_id_k_entry:Option<KEntry>,
	/// has one
	pub ad_session_id_ad_session:Option<AdSession>,
}

pub struct KEntry {
	/// primary
	/// not nullable 
	pub k_entry_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_session_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub descriptionurl:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub ispublic:String,
	pub k_source_id:Option<f64>,
	/// not nullable 
	pub k_topic_id:f64,
	pub keywords:Option<String>,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub rating:f64,
	/// not nullable 
	pub textmsg:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub validto:Option<NaiveDateTime>,
	/// has one
	pub k_topic_id_k_topic:Option<KTopic>,
	/// has one
	pub k_source_id_k_source:Option<KSource>,
	/// has one
	pub ad_session_id_ad_session:Option<AdSession>,
	/// has one, extension table
	pub k_entryrelated:Option<Box<KEntryrelated>>,
	/// has one, extension table
	pub k_entryrelated:Option<Box<KEntryrelated>>,
	/// has many
	pub k_comment:Option<Vec<KComment>>,
	/// has many
	pub k_entrycategory:Option<Vec<KEntrycategory>>,
}

pub struct KEntrycategory {
	/// primary
	/// not nullable 
	pub k_category_id:f64,
	/// primary
	/// not nullable 
	pub k_entry_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub k_categoryvalue_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub k_category_id_k_category:Option<KCategory>,
	/// has one
	pub k_entry_id_k_entry:Option<KEntry>,
	/// has one
	pub k_categoryvalue_id_k_categoryvalue:Option<KCategoryvalue>,
}

pub struct KEntryrelated {
	/// primary
	/// not nullable 
	pub k_entry_id:f64,
	/// primary
	/// not nullable 
	pub k_entryrelated_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub name:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub k_entry_id_k_entry:Option<KEntry>,
	/// has one
	pub k_entryrelated_id_k_entry:Option<KEntry>,
}

pub struct KIndex {
	/// primary
	/// not nullable 
	pub k_index_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_table_id:f64,
	pub c_doctype_id:Option<f64>,
	pub cm_webproject_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub excerpt:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub keyword:String,
	pub r_requesttype_id:Option<f64>,
	/// not nullable 
	pub record_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub sourceupdated:NaiveDateTime,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_table_id_ad_table:Option<AdTable>,
	/// has one
	pub cm_webproject_id_cm_webproject:Option<CmWebproject>,
	/// has one
	pub r_requesttype_id_r_requesttype:Option<RRequesttype>,
	/// has one
	pub c_doctype_id_c_doctype:Option<CDoctype>,
}

pub struct KIndexlog {
	/// primary
	/// not nullable 
	pub k_indexlog_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub indexquery:String,
	/// defaults to: 0
	/// not nullable 
	pub indexqueryresult:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub querysource:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
}

pub struct KIndexstop {
	/// primary
	/// not nullable 
	pub k_indexstop_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub c_doctype_id:Option<f64>,
	pub cm_webproject_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub ismanual:String,
	/// not nullable 
	pub keyword:String,
	pub r_requesttype_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub cm_webproject_id_cm_webproject:Option<CmWebproject>,
	/// has one
	pub r_requesttype_id_r_requesttype:Option<RRequesttype>,
	/// has one
	pub c_doctype_id_c_doctype:Option<CDoctype>,
}

pub struct KSource {
	/// primary
	/// not nullable 
	pub k_source_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub descriptionurl:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has many
	pub k_entry:Option<Vec<KEntry>>,
}

pub struct KSynonym {
	/// primary
	/// not nullable 
	pub k_synonym_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_language:String,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub synonymname:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_language_ad_language:Option<AdLanguage>,
}

pub struct KTopic {
	/// primary
	/// not nullable 
	pub k_topic_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub ispublic:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub ispublicwrite:String,
	/// not nullable 
	pub k_type_id:f64,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub k_type_id_k_type:Option<KType>,
	/// has many
	pub k_entry:Option<Vec<KEntry>>,
}

pub struct KType {
	/// primary
	/// not nullable 
	pub k_type_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub ispublic:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub ispublicwrite:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has many
	pub k_topic:Option<Vec<KTopic>>,
}

pub struct Length {
	/// primary
	/// not nullable 
	pub length_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: NULL::numeric
	pub datatype_id:Option<f64>,
	/// defaults to: NULL::character varying
	pub description:Option<String>,
	/// defaults to: NULL::character varying
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub length:Option<f64>,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
}

pub struct MAttribute {
	/// primary
	/// not nullable 
	pub m_attribute_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: 'S'::bpchar
	/// not nullable 
	pub attributevaluetype:String,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isinstanceattribute:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub ismandatory:String,
	pub m_attributesearch_id:Option<f64>,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub m_attributesearch_id_m_attributesearch:Option<MAttributesearch>,
	/// has many
	pub m_attributeinstance:Option<Vec<MAttributeinstance>>,
	/// has many
	pub m_attributeuse:Option<Vec<MAttributeuse>>,
	/// has many
	pub m_attributevalue:Option<Vec<MAttributevalue>>,
	/// has many
	pub qm_specificationline:Option<Vec<QmSpecificationline>>,
}

pub struct MAttributeinstance {
	/// primary
	/// defaults to: 0
	/// not nullable 
	pub m_attributesetinstance_id:f64,
	/// primary
	/// not nullable 
	pub m_attribute_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub m_attributevalue_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub value:Option<String>,
	pub valuenumber:Option<f64>,
	/// has one
	pub m_attributesetinstance_id_m_attributesetinstance:Option<MAttributesetinstance>,
	/// has one
	pub m_attribute_id_m_attribute:Option<MAttribute>,
	/// has one
	pub m_attributevalue_id_m_attributevalue:Option<MAttributevalue>,
}

pub struct MAttributesearch {
	/// primary
	/// not nullable 
	pub m_attributesearch_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has many
	pub m_attribute:Option<Vec<MAttribute>>,
}

pub struct MAttributeset {
	/// primary
	/// not nullable 
	pub m_attributeset_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub guaranteedays:Option<f64>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isguaranteedate:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isguaranteedatemandatory:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isinstanceattribute:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub islot:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub islotmandatory:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isserno:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub issernomandatory:String,
	pub lotchareoverwrite:Option<String>,
	pub lotcharsoverwrite:Option<String>,
	pub m_lotctl_id:Option<f64>,
	pub m_sernoctl_id:Option<f64>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub mandatorytype:String,
	/// not nullable 
	pub name:String,
	pub sernochareoverwrite:Option<String>,
	pub sernocharsoverwrite:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub m_sernoctl_id_m_sernoctl:Option<MSernoctl>,
	/// has one
	pub m_lotctl_id_m_lotctl:Option<MLotctl>,
	/// has many
	pub m_attributesetexclude:Option<Vec<MAttributesetexclude>>,
	/// has many
	pub m_attributesetinstance:Option<Vec<MAttributesetinstance>>,
	/// has many
	pub m_attributeuse:Option<Vec<MAttributeuse>>,
	/// has many
	pub m_product:Option<Vec<MProduct>>,
	/// has many
	pub qm_specification:Option<Vec<QmSpecification>>,
}

pub struct MAttributesetexclude {
	/// primary
	/// not nullable 
	pub m_attributesetexclude_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_table_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub issotrx:String,
	/// not nullable 
	pub m_attributeset_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub m_attributeset_id_m_attributeset:Option<MAttributeset>,
	/// has one
	pub ad_table_id_ad_table:Option<AdTable>,
}

pub struct MAttributesetinstance {
	/// primary
	/// not nullable 
	pub m_attributesetinstance_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub guaranteedate:Option<NaiveDateTime>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub lot:Option<String>,
	pub m_attributeset_id:Option<f64>,
	pub m_lot_id:Option<f64>,
	pub serno:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub m_attributeset_id_m_attributeset:Option<MAttributeset>,
	/// has one
	pub m_lot_id_m_lot:Option<MLot>,
	/// has many
	pub a_asset:Option<Vec<AAsset>>,
	/// has many
	pub c_invoiceline:Option<Vec<CInvoiceline>>,
	/// has many
	pub c_landedcostallocation:Option<Vec<CLandedcostallocation>>,
	/// has many
	pub c_orderline:Option<Vec<COrderline>>,
	/// has many
	pub c_projectissue:Option<Vec<CProjectissue>>,
	/// has many
	pub c_projectissuema:Option<Vec<CProjectissuema>>,
	/// has many
	pub c_rfqline:Option<Vec<CRfqline>>,
	/// has many
	pub m_attributeinstance:Option<Vec<MAttributeinstance>>,
	/// has many
	pub m_bomproduct:Option<Vec<MBomproduct>>,
	/// has many
	pub m_cost:Option<Vec<MCost>>,
	/// has many
	pub m_costdetail:Option<Vec<MCostdetail>>,
	/// has many
	pub m_costqueue:Option<Vec<MCostqueue>>,
	/// has many
	pub m_inoutline:Option<Vec<MInoutline>>,
	/// has many
	pub m_inoutlinema:Option<Vec<MInoutlinema>>,
	/// has many
	pub m_inventoryline:Option<Vec<MInventoryline>>,
	/// has many
	pub m_inventorylinema:Option<Vec<MInventorylinema>>,
	/// has many
	pub m_movementline:Option<Vec<MMovementline>>,
	/// has many
	pub m_movementlinema:Option<Vec<MMovementlinema>>,
	/// has many
	pub m_product:Option<Vec<MProduct>>,
	/// has many
	pub m_productionline:Option<Vec<MProductionline>>,
	/// has many
	pub m_productionlinema:Option<Vec<MProductionlinema>>,
	/// has many
	pub m_storage:Option<Vec<MStorage>>,
	/// has many
	pub m_transaction:Option<Vec<MTransaction>>,
	/// has many
	pub m_transactionallocation:Option<Vec<MTransactionallocation>>,
	/// has many
	pub t_inventoryvalue:Option<Vec<TInventoryvalue>>,
	/// has many
	pub t_transaction:Option<Vec<TTransaction>>,
}

pub struct MAttributeuse {
	/// primary
	/// not nullable 
	pub m_attribute_id:f64,
	/// primary
	/// not nullable 
	pub m_attributeset_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub seqno:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub m_attribute_id_m_attribute:Option<MAttribute>,
	/// has one
	pub m_attributeset_id_m_attributeset:Option<MAttributeset>,
}

pub struct MAttributevalue {
	/// primary
	/// not nullable 
	pub m_attributevalue_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub m_attribute_id:f64,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
	/// has one
	pub m_attribute_id_m_attribute:Option<MAttribute>,
	/// has many
	pub m_attributeinstance:Option<Vec<MAttributeinstance>>,
}

pub struct MBom {
	/// primary
	/// not nullable 
	pub m_bom_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub bomtype:String,
	/// not nullable 
	pub bomuse:String,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub m_changenotice_id:Option<f64>,
	/// not nullable 
	pub m_product_id:f64,
	/// not nullable 
	pub name:String,
	pub processing:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub m_changenotice_id_m_changenotice:Option<MChangenotice>,
	/// has many
	pub m_bomproduct:Option<Vec<MBomproduct>>,
	/// has many
	pub r_group:Option<Vec<RGroup>>,
}

pub struct MBomalternative {
	/// primary
	/// not nullable 
	pub m_bomalternative_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub m_product_id:f64,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has many
	pub m_bomproduct:Option<Vec<MBomproduct>>,
}

pub struct MBomproduct {
	/// primary
	/// not nullable 
	pub m_bomproduct_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub bomproducttype:String,
	/// defaults to: 0
	/// not nullable 
	pub bomqty:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isphantom:String,
	/// defaults to: 0
	/// not nullable 
	pub leadtimeoffset:f64,
	/// defaults to: 0
	/// not nullable 
	pub line:f64,
	pub m_attributesetinstance_id:Option<f64>,
	/// not nullable 
	pub m_bom_id:f64,
	pub m_bomalternative_id:Option<f64>,
	pub m_changenotice_id:Option<f64>,
	pub m_productbom_id:Option<f64>,
	pub m_productoperation_id:Option<f64>,
	/// defaults to: 0
	pub seqno:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub m_bom_id_m_bom:Option<MBom>,
	/// has one
	pub m_productbom_id_m_product:Option<MProduct>,
	/// has one
	pub m_changenotice_id_m_changenotice:Option<MChangenotice>,
	/// has one
	pub m_attributesetinstance_id_m_attributesetinstance:Option<MAttributesetinstance>,
	/// has one
	pub m_bomalternative_id_m_bomalternative:Option<MBomalternative>,
	/// has one
	pub m_productoperation_id_m_productoperation:Option<MProductoperation>,
}

pub struct MChangenotice {
	/// primary
	/// not nullable 
	pub m_changenotice_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub detailinfo:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isapproved:String,
	/// not nullable 
	pub name:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	pub processing:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has many
	pub dd_networkdistribution:Option<Vec<DdNetworkdistribution>>,
	/// has many
	pub m_bom:Option<Vec<MBom>>,
	/// has many
	pub m_bomproduct:Option<Vec<MBomproduct>>,
	/// has many
	pub m_changerequest:Option<Vec<MChangerequest>>,
	/// has many
	pub pp_order_bom:Option<Vec<PpOrderBom>>,
	/// has many
	pub pp_order_bomline:Option<Vec<PpOrderBomline>>,
	/// has many
	pub pp_product_bom:Option<Vec<PpProductBom>>,
	/// has many
	pub pp_product_bomline:Option<Vec<PpProductBomline>>,
	/// has many
	pub r_group:Option<Vec<RGroup>>,
	/// has many
	pub r_request:Option<Vec<RRequest>>,
}

pub struct MChangerequest {
	/// primary
	/// not nullable 
	pub m_changerequest_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub detailinfo:Option<String>,
	/// not nullable 
	pub documentno:String,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isapproved:String,
	pub m_changenotice_id:Option<f64>,
	pub m_fixchangenotice_id:Option<f64>,
	/// not nullable 
	pub name:String,
	pub pp_product_bom_id:Option<f64>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub m_changenotice_id_m_changenotice:Option<MChangenotice>,
	/// has one
	pub m_fixchangenotice_id_m_changenotice:Option<MChangenotice>,
	/// has many
	pub r_request:Option<Vec<RRequest>>,
}

pub struct MCost {
	/// primary
	/// not nullable 
	pub ad_client_id:f64,
	/// primary
	/// not nullable 
	pub ad_org_id:f64,
	/// primary
	/// not nullable 
	pub m_product_id:f64,
	/// primary
	/// not nullable 
	pub m_costtype_id:f64,
	/// primary
	/// not nullable 
	pub c_acctschema_id:f64,
	/// primary
	/// not nullable 
	pub m_costelement_id:f64,
	/// primary
	/// not nullable 
	pub m_attributesetinstance_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 0
	pub cumulatedamt:Option<f64>,
	/// defaults to: 0
	pub cumulatedqty:Option<f64>,
	/// not nullable 
	pub currentcostprice:f64,
	/// defaults to: 0
	pub currentcostpricell:Option<f64>,
	/// defaults to: 0
	/// not nullable 
	pub currentqty:f64,
	pub description:Option<String>,
	pub futurecostprice:Option<f64>,
	pub futurecostpricell:Option<f64>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	pub iscostfrozen:Option<String>,
	/// defaults to: 0
	pub percent:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_client_id_ad_client:Option<AdClient>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub m_costtype_id_m_costtype:Option<MCosttype>,
	/// has one
	pub c_acctschema_id_c_acctschema:Option<CAcctschema>,
	/// has one
	pub m_costelement_id_m_costelement:Option<MCostelement>,
	/// has one
	pub m_attributesetinstance_id_m_attributesetinstance:Option<MAttributesetinstance>,
}

pub struct MCostdetail {
	/// primary
	/// not nullable 
	pub m_costdetail_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: 0
	/// not nullable 
	pub amt:f64,
	/// not nullable 
	pub c_acctschema_id:f64,
	pub c_invoiceline_id:Option<f64>,
	pub c_orderline_id:Option<f64>,
	pub c_projectissue_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 0
	pub deltaamt:Option<f64>,
	/// defaults to: 0
	pub deltaqty:Option<f64>,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub issotrx:String,
	/// not nullable 
	pub m_attributesetinstance_id:f64,
	pub m_costelement_id:Option<f64>,
	pub m_inoutline_id:Option<f64>,
	pub m_inventoryline_id:Option<f64>,
	pub m_movementline_id:Option<f64>,
	/// not nullable 
	pub m_product_id:f64,
	pub m_productionline_id:Option<f64>,
	pub pp_cost_collector_id:Option<f64>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	/// defaults to: 0
	/// not nullable 
	pub qty:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_client_id_ad_client:Option<AdClient>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub c_acctschema_id_c_acctschema:Option<CAcctschema>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub m_attributesetinstance_id_m_attributesetinstance:Option<MAttributesetinstance>,
	/// has one
	pub m_costelement_id_m_costelement:Option<MCostelement>,
	/// has one
	pub c_orderline_id_c_orderline:Option<COrderline>,
	/// has one
	pub m_inoutline_id_m_inoutline:Option<MInoutline>,
	/// has one
	pub c_invoiceline_id_c_invoiceline:Option<CInvoiceline>,
	/// has one
	pub m_movementline_id_m_movementline:Option<MMovementline>,
	/// has one
	pub m_inventoryline_id_m_inventoryline:Option<MInventoryline>,
	/// has one
	pub m_productionline_id_m_productionline:Option<MProductionline>,
	/// has one
	pub c_projectissue_id_c_projectissue:Option<CProjectissue>,
}

pub struct MCostelement {
	/// primary
	/// not nullable 
	pub m_costelement_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub costelementtype:String,
	pub costingmethod:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub iscalculated:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has many
	pub c_landedcost:Option<Vec<CLandedcost>>,
	/// has many
	pub c_landedcostallocation:Option<Vec<CLandedcostallocation>>,
	/// has many
	pub m_cost:Option<Vec<MCost>>,
	/// has many
	pub m_costdetail:Option<Vec<MCostdetail>>,
	/// has many
	pub m_costqueue:Option<Vec<MCostqueue>>,
	/// has many
	pub pp_order_cost:Option<Vec<PpOrderCost>>,
	/// has many
	pub t_inventoryvalue:Option<Vec<TInventoryvalue>>,
}

pub struct MCostqueue {
	/// primary
	/// not nullable 
	pub m_costqueue_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_acctschema_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 0
	/// not nullable 
	pub currentcostprice:f64,
	/// defaults to: 0
	/// not nullable 
	pub currentqty:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub m_attributesetinstance_id:f64,
	/// not nullable 
	pub m_costelement_id:f64,
	/// not nullable 
	pub m_costtype_id:f64,
	/// not nullable 
	pub m_product_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub m_costtype_id_m_costtype:Option<MCosttype>,
	/// has one
	pub c_acctschema_id_c_acctschema:Option<CAcctschema>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub m_attributesetinstance_id_m_attributesetinstance:Option<MAttributesetinstance>,
	/// has one
	pub m_costelement_id_m_costelement:Option<MCostelement>,
}

pub struct MCosttype {
	/// primary
	/// not nullable 
	pub m_costtype_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has many
	pub c_acctschema:Option<Vec<CAcctschema>>,
	/// has many
	pub m_cost:Option<Vec<MCost>>,
	/// has many
	pub m_costqueue:Option<Vec<MCostqueue>>,
	/// has many
	pub pp_order_cost:Option<Vec<PpOrderCost>>,
}

pub struct MDemand {
	/// primary
	/// not nullable 
	pub m_demand_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_calendar_id:f64,
	/// not nullable 
	pub c_year_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isdefault:String,
	/// not nullable 
	pub name:String,
	pub processing:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_calendar_id_c_calendar:Option<CCalendar>,
	/// has one
	pub c_year_id_c_year:Option<CYear>,
	/// has many
	pub m_demandline:Option<Vec<MDemandline>>,
}

pub struct MDemanddetail {
	/// primary
	/// not nullable 
	pub m_demanddetail_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub c_orderline_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub m_demandline_id:f64,
	pub m_forecastline_id:Option<f64>,
	pub m_requisitionline_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub m_demandline_id_m_demandline:Option<MDemandline>,
	/// has one
	pub m_forecastline_id_m_forecastline:Option<MForecastline>,
	/// has one
	pub m_requisitionline_id_m_requisitionline:Option<MRequisitionline>,
	/// has one
	pub c_orderline_id_c_orderline:Option<COrderline>,
}

pub struct MDemandline {
	/// primary
	/// not nullable 
	pub m_demandline_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_period_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub m_demand_id:f64,
	/// not nullable 
	pub m_product_id:f64,
	/// defaults to: 0
	/// not nullable 
	pub qty:f64,
	/// defaults to: 0
	/// not nullable 
	pub qtycalculated:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub m_demand_id_m_demand:Option<MDemand>,
	/// has one
	pub c_period_id_c_period:Option<CPeriod>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has many
	pub m_demanddetail:Option<Vec<MDemanddetail>>,
}

pub struct MDiscountschema {
	/// primary
	/// not nullable 
	pub m_discountschema_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub cumulativelevel:Option<String>,
	pub description:Option<String>,
	/// not nullable 
	pub discounttype:String,
	pub flatdiscount:Option<f64>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isbpartnerflatdiscount:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isquantitybased:String,
	/// not nullable 
	pub name:String,
	pub processing:Option<String>,
	pub script:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub validfrom:NaiveDateTime,
	/// has many
	pub c_bp_group:Option<Vec<CBpGroup>>,
	/// has many
	pub c_bpartner:Option<Vec<CBpartner>>,
	/// has many
	pub m_discountschemabreak:Option<Vec<MDiscountschemabreak>>,
	/// has many
	pub m_discountschemaline:Option<Vec<MDiscountschemaline>>,
	/// has many
	pub m_pricelist_version:Option<Vec<MPricelistVersion>>,
}

pub struct MDiscountschemabreak {
	/// primary
	/// not nullable 
	pub m_discountschemabreak_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub breakdiscount:f64,
	/// not nullable 
	pub breakvalue:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isbpartnerflatdiscount:String,
	/// not nullable 
	pub m_discountschema_id:f64,
	pub m_product_category_id:Option<f64>,
	pub m_product_id:Option<f64>,
	/// not nullable 
	pub seqno:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub m_discountschema_id_m_discountschema:Option<MDiscountschema>,
	/// has one
	pub m_product_category_id_m_product_category:Option<MProductCategory>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
}

pub struct MDiscountschemaline {
	/// primary
	/// not nullable 
	pub m_discountschemaline_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub c_bpartner_id:Option<f64>,
	/// not nullable 
	pub c_conversiontype_id:f64,
	pub classification:Option<String>,
	/// not nullable 
	pub conversiondate:NaiveDateTime,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub group1:Option<String>,
	pub group2:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 0
	/// not nullable 
	pub limit_addamt:f64,
	/// not nullable 
	pub limit_base:String,
	/// not nullable 
	pub limit_discount:f64,
	/// defaults to: 0
	pub limit_fixed:Option<f64>,
	/// defaults to: 0
	/// not nullable 
	pub limit_maxamt:f64,
	/// defaults to: 0
	/// not nullable 
	pub limit_minamt:f64,
	/// not nullable 
	pub limit_rounding:String,
	/// defaults to: 0
	/// not nullable 
	pub list_addamt:f64,
	/// not nullable 
	pub list_base:String,
	/// not nullable 
	pub list_discount:f64,
	/// defaults to: 0
	pub list_fixed:Option<f64>,
	/// defaults to: 0
	/// not nullable 
	pub list_maxamt:f64,
	/// defaults to: 0
	/// not nullable 
	pub list_minamt:f64,
	/// not nullable 
	pub list_rounding:String,
	/// not nullable 
	pub m_discountschema_id:f64,
	pub m_product_category_id:Option<f64>,
	pub m_product_id:Option<f64>,
	/// not nullable 
	pub seqno:f64,
	/// defaults to: 0
	/// not nullable 
	pub std_addamt:f64,
	/// not nullable 
	pub std_base:String,
	/// not nullable 
	pub std_discount:f64,
	/// defaults to: 0
	pub std_fixed:Option<f64>,
	/// defaults to: 0
	/// not nullable 
	pub std_maxamt:f64,
	/// defaults to: 0
	/// not nullable 
	pub std_minamt:f64,
	/// not nullable 
	pub std_rounding:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub m_discountschema_id_m_discountschema:Option<MDiscountschema>,
	/// has one
	pub m_product_category_id_m_product_category:Option<MProductCategory>,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub c_conversiontype_id_c_conversiontype:Option<CConversiontype>,
}

pub struct MDistributionlist {
	/// primary
	/// not nullable 
	pub m_distributionlist_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	pub processing:Option<String>,
	pub ratiototal:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has many
	pub m_distributionlistline:Option<Vec<MDistributionlistline>>,
	/// has many
	pub m_distributionrunline:Option<Vec<MDistributionrunline>>,
	/// has many
	pub t_distributionrundetail:Option<Vec<TDistributionrundetail>>,
}

pub struct MDistributionlistline {
	/// primary
	/// not nullable 
	pub m_distributionlistline_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_bpartner_id:f64,
	/// not nullable 
	pub c_bpartner_location_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub m_distributionlist_id:f64,
	/// defaults to: 0
	/// not nullable 
	pub minqty:f64,
	pub ratio:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub m_distributionlist_id_m_distributionlist:Option<MDistributionlist>,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub c_bpartner_location_id_c_bpartner_location:Option<CBpartnerLocation>,
	/// has many
	pub t_distributionrundetail:Option<Vec<TDistributionrundetail>>,
}

pub struct MDistributionrun {
	/// primary
	/// not nullable 
	pub m_distributionrun_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub c_bpartner_id:Option<f64>,
	pub c_bpartner_location_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub iscreatesingleorder:String,
	/// not nullable 
	pub name:String,
	pub processing:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub c_bpartner_location_id_c_bpartner_location:Option<CBpartnerLocation>,
	/// has many
	pub m_distributionrunline:Option<Vec<MDistributionrunline>>,
	/// has many
	pub t_distributionrundetail:Option<Vec<TDistributionrundetail>>,
}

pub struct MDistributionrunline {
	/// primary
	/// not nullable 
	pub m_distributionrunline_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub line:f64,
	/// not nullable 
	pub m_distributionlist_id:f64,
	/// not nullable 
	pub m_distributionrun_id:f64,
	/// not nullable 
	pub m_product_id:f64,
	/// defaults to: 0
	/// not nullable 
	pub minqty:f64,
	/// defaults to: 0
	/// not nullable 
	pub totalqty:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub m_distributionrun_id_m_distributionrun:Option<MDistributionrun>,
	/// has one
	pub m_distributionlist_id_m_distributionlist:Option<MDistributionlist>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has many
	pub t_distributionrundetail:Option<Vec<TDistributionrundetail>>,
}

pub struct MEdi {
	/// primary
	/// not nullable 
	pub m_edi_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_bp_edi_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub documentno:String,
	/// not nullable 
	pub edistatus:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub line:f64,
	/// not nullable 
	pub m_product_id:f64,
	/// not nullable 
	pub m_warehouse_id:f64,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	/// defaults to: 0
	pub reply_price:Option<f64>,
	/// defaults to: 0
	pub reply_qtyavailable:Option<f64>,
	/// defaults to: 0
	pub reply_qtyconfirmed:Option<f64>,
	pub reply_received:Option<NaiveDateTime>,
	pub reply_remarks:Option<String>,
	pub reply_shipdate:Option<NaiveDateTime>,
	/// defaults to: 0
	pub request_price:Option<f64>,
	/// defaults to: 0
	/// not nullable 
	pub request_qty:f64,
	/// not nullable 
	pub request_shipdate:NaiveDateTime,
	pub trxreceived:Option<NaiveDateTime>,
	/// not nullable 
	pub trxsent:NaiveDateTime,
	/// not nullable 
	pub trxtype:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_bp_edi_id_c_bp_edi:Option<CBpEdi>,
	/// has one
	pub m_warehouse_id_m_warehouse:Option<MWarehouse>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has many
	pub m_edi_info:Option<Vec<MEdiInfo>>,
}

pub struct MEdiInfo {
	/// primary
	/// not nullable 
	pub m_edi_info_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub info:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub m_edi_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub m_edi_id_m_edi:Option<MEdi>,
}

pub struct MForecast {
	/// primary
	/// not nullable 
	pub m_forecast_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_calendar_id:f64,
	/// not nullable 
	pub c_year_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isdefault:String,
	pub m_pricelist_id:Option<f64>,
	/// not nullable 
	pub name:String,
	pub processing:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_calendar_id_c_calendar:Option<CCalendar>,
	/// has one
	pub c_year_id_c_year:Option<CYear>,
	/// has many
	pub m_forecastline:Option<Vec<MForecastline>>,
	/// has many
	pub pp_mrp:Option<Vec<PpMrp>>,
}

pub struct MForecastline {
	/// primary
	/// not nullable 
	pub m_forecastline_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_period_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub datepromised:NaiveDateTime,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub m_forecast_id:f64,
	/// not nullable 
	pub m_product_id:f64,
	/// not nullable 
	pub m_warehouse_id:f64,
	/// defaults to: 0
	/// not nullable 
	pub qty:f64,
	/// defaults to: 0
	/// not nullable 
	pub qtycalculated:f64,
	pub salesrep_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub m_forecast_id_m_forecast:Option<MForecast>,
	/// has one
	pub c_period_id_c_period:Option<CPeriod>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub m_warehouse_id_m_warehouse:Option<MWarehouse>,
	/// has many
	pub m_demanddetail:Option<Vec<MDemanddetail>>,
	/// has many
	pub pp_mrp:Option<Vec<PpMrp>>,
}

pub struct MFreight {
	/// primary
	/// not nullable 
	pub m_freight_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub c_country_id:Option<f64>,
	/// not nullable 
	pub c_currency_id:f64,
	pub c_region_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 0
	/// not nullable 
	pub freightamt:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub m_freightcategory_id:f64,
	/// not nullable 
	pub m_shipper_id:f64,
	pub to_country_id:Option<f64>,
	pub to_region_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub validfrom:NaiveDateTime,
	/// has one
	pub m_shipper_id_m_shipper:Option<MShipper>,
	/// has one
	pub m_freightcategory_id_m_freightcategory:Option<MFreightcategory>,
	/// has one
	pub c_country_id_c_country:Option<CCountry>,
	/// has one
	pub to_country_id_c_country:Option<CCountry>,
	/// has one
	pub c_region_id_c_region:Option<CRegion>,
	/// has one
	pub to_region_id_c_region:Option<CRegion>,
	/// has one
	pub c_currency_id_c_currency:Option<CCurrency>,
}

pub struct MFreightcategory {
	/// primary
	/// not nullable 
	pub m_freightcategory_id:f64,
	/// unique
	/// not nullable 
	pub ad_client_id:f64,
	/// unique
	/// not nullable 
	pub value:String,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has many
	pub c_order:Option<Vec<COrder>>,
	/// has many
	pub m_freight:Option<Vec<MFreight>>,
	/// has many
	pub m_product:Option<Vec<MProduct>>,
}

pub struct MInout {
	/// primary
	/// not nullable 
	pub m_inout_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_orgtrx_id:Option<f64>,
	pub ad_user_id:Option<f64>,
	pub c_activity_id:Option<f64>,
	/// not nullable 
	pub c_bpartner_id:f64,
	/// not nullable 
	pub c_bpartner_location_id:f64,
	pub c_campaign_id:Option<f64>,
	pub c_charge_id:Option<f64>,
	/// not nullable 
	pub c_doctype_id:f64,
	pub c_invoice_id:Option<f64>,
	pub c_order_id:Option<f64>,
	pub c_project_id:Option<f64>,
	/// defaults to: 0
	pub chargeamt:Option<f64>,
	pub createconfirm:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub createfrom:Option<String>,
	pub createpackage:Option<String>,
	/// not nullable 
	pub dateacct:NaiveDateTime,
	pub dateordered:Option<NaiveDateTime>,
	pub dateprinted:Option<NaiveDateTime>,
	pub datereceived:Option<NaiveDateTime>,
	/// not nullable 
	pub deliveryrule:String,
	/// not nullable 
	pub deliveryviarule:String,
	pub description:Option<String>,
	/// not nullable 
	pub docaction:String,
	/// not nullable 
	pub docstatus:String,
	/// not nullable 
	pub documentno:String,
	pub dropship_bpartner_id:Option<f64>,
	pub dropship_location_id:Option<f64>,
	pub dropship_user_id:Option<f64>,
	/// defaults to: 0
	pub freightamt:Option<f64>,
	/// not nullable 
	pub freightcostrule:String,
	pub generateto:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isapproved:String,
	/// defaults to: 'N'::bpchar
	pub isdropship:Option<String>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isindispute:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isintransit:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isprinted:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub issotrx:String,
	pub m_rma_id:Option<f64>,
	pub m_shipper_id:Option<f64>,
	/// not nullable 
	pub m_warehouse_id:f64,
	/// not nullable 
	pub movementdate:NaiveDateTime,
	/// not nullable 
	pub movementtype:String,
	pub nopackages:Option<f64>,
	pub pickdate:Option<NaiveDateTime>,
	pub poreference:Option<String>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub posted:String,
	/// not nullable 
	pub priorityrule:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	pub processing:Option<String>,
	pub ref_inout_id:Option<f64>,
	pub reversal_id:Option<f64>,
	pub salesrep_id:Option<f64>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub sendemail:String,
	pub shipdate:Option<NaiveDateTime>,
	pub trackingno:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub user1_id:Option<f64>,
	pub user2_id:Option<f64>,
	pub volume:Option<f64>,
	pub weight:Option<f64>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub c_doctype_id_c_doctype:Option<CDoctype>,
	/// has one
	pub c_order_id_c_order:Option<COrder>,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub c_bpartner_location_id_c_bpartner_location:Option<CBpartnerLocation>,
	/// has one
	pub m_warehouse_id_m_warehouse:Option<MWarehouse>,
	/// has one
	pub m_shipper_id_m_shipper:Option<MShipper>,
	/// has one
	pub c_charge_id_c_charge:Option<CCharge>,
	/// has one
	pub c_invoice_id_c_invoice:Option<CInvoice>,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
	/// has one
	pub salesrep_id_ad_user:Option<AdUser>,
	/// has one
	pub ad_orgtrx_id_ad_org:Option<AdOrg>,
	/// has one
	pub c_project_id_c_project:Option<CProject>,
	/// has one
	pub c_campaign_id_c_campaign:Option<CCampaign>,
	/// has one
	pub c_activity_id_c_activity:Option<CActivity>,
	/// has one
	pub user1_id_c_elementvalue:Option<CElementvalue>,
	/// has one
	pub user2_id_c_elementvalue:Option<CElementvalue>,
	/// has one, self referential
	pub ref_inout_id_m_inout:Option<Box<MInout>>,
	/// has one
	pub m_rma_id_m_rma:Option<MRma>,
	/// has one, self referential
	pub reversal_id_m_inout:Option<Box<MInout>>,
	/// has many
	pub c_landedcost:Option<Vec<CLandedcost>>,
	/// has many
	pub m_inout:Option<Vec<MInout>>,
	/// has many
	pub m_inoutconfirm:Option<Vec<MInoutconfirm>>,
	/// has many
	pub m_inoutline:Option<Vec<MInoutline>>,
	/// has many
	pub m_package:Option<Vec<MPackage>>,
	/// has many
	pub m_rma:Option<Vec<MRma>>,
	/// has many
	pub r_request:Option<Vec<RRequest>>,
	/// has many
	pub r_requestaction:Option<Vec<RRequestaction>>,
}

pub struct MInoutconfirm {
	/// primary
	/// not nullable 
	pub m_inoutconfirm_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub approvalamt:Option<f64>,
	pub c_invoice_id:Option<f64>,
	pub confirmationno:Option<String>,
	/// not nullable 
	pub confirmtype:String,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub createpackage:Option<String>,
	pub description:Option<String>,
	/// not nullable 
	pub docaction:String,
	/// not nullable 
	pub docstatus:String,
	/// not nullable 
	pub documentno:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isapproved:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub iscancelled:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isindispute:String,
	/// not nullable 
	pub m_inout_id:f64,
	pub m_inventory_id:Option<f64>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	pub processing:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub m_inout_id_m_inout:Option<MInout>,
	/// has one
	pub m_inventory_id_m_inventory:Option<MInventory>,
	/// has one
	pub c_invoice_id_c_invoice:Option<CInvoice>,
	/// has many
	pub m_inoutlineconfirm:Option<Vec<MInoutlineconfirm>>,
}

pub struct MInoutline {
	/// primary
	/// not nullable 
	pub m_inoutline_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_orgtrx_id:Option<f64>,
	pub c_activity_id:Option<f64>,
	pub c_campaign_id:Option<f64>,
	pub c_charge_id:Option<f64>,
	pub c_orderline_id:Option<f64>,
	pub c_project_id:Option<f64>,
	pub c_projectphase_id:Option<f64>,
	pub c_projecttask_id:Option<f64>,
	/// not nullable 
	pub c_uom_id:f64,
	/// defaults to: 0
	pub confirmedqty:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isdescription:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isinvoiced:String,
	/// not nullable 
	pub line:f64,
	/// defaults to: 0
	pub m_attributesetinstance_id:Option<f64>,
	/// not nullable 
	pub m_inout_id:f64,
	pub m_locator_id:Option<f64>,
	pub m_product_id:Option<f64>,
	pub m_rmaline_id:Option<f64>,
	/// defaults to: 0
	/// not nullable 
	pub movementqty:f64,
	/// defaults to: 0
	pub pickedqty:Option<f64>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	/// not nullable 
	pub qtyentered:f64,
	pub ref_inoutline_id:Option<f64>,
	pub reversalline_id:Option<f64>,
	/// defaults to: 0
	pub scrappedqty:Option<f64>,
	/// defaults to: 0
	pub targetqty:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub user1_id:Option<f64>,
	pub user2_id:Option<f64>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub m_inout_id_m_inout:Option<MInout>,
	/// has one
	pub c_orderline_id_c_orderline:Option<COrderline>,
	/// has one
	pub m_locator_id_m_locator:Option<MLocator>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub c_uom_id_c_uom:Option<CUom>,
	/// has one
	pub m_attributesetinstance_id_m_attributesetinstance:Option<MAttributesetinstance>,
	/// has one, self referential
	pub ref_inoutline_id_m_inoutline:Option<Box<MInoutline>>,
	/// has one
	pub c_charge_id_c_charge:Option<CCharge>,
	/// has one
	pub c_project_id_c_project:Option<CProject>,
	/// has one
	pub c_projectphase_id_c_projectphase:Option<CProjectphase>,
	/// has one
	pub c_projecttask_id_c_projecttask:Option<CProjecttask>,
	/// has one
	pub c_campaign_id_c_campaign:Option<CCampaign>,
	/// has one
	pub c_activity_id_c_activity:Option<CActivity>,
	/// has one
	pub user1_id_c_elementvalue:Option<CElementvalue>,
	/// has one
	pub user2_id_c_elementvalue:Option<CElementvalue>,
	/// has one
	pub ad_orgtrx_id_ad_org:Option<AdOrg>,
	/// has one
	pub m_rmaline_id_m_rmaline:Option<MRmaline>,
	/// has many
	pub a_asset:Option<Vec<AAsset>>,
	/// has many
	pub a_asset_delivery:Option<Vec<AAssetDelivery>>,
	/// has many
	pub c_invoiceline:Option<Vec<CInvoiceline>>,
	/// has many
	pub c_landedcost:Option<Vec<CLandedcost>>,
	/// has many
	pub c_projectissue:Option<Vec<CProjectissue>>,
	/// has many
	pub m_costdetail:Option<Vec<MCostdetail>>,
	/// has many
	pub m_inoutline:Option<Vec<MInoutline>>,
	/// has many
	pub m_inoutlineconfirm:Option<Vec<MInoutlineconfirm>>,
	/// has many
	pub m_inoutlinema:Option<Vec<MInoutlinema>>,
	/// has many
	pub m_matchinv:Option<Vec<MMatchinv>>,
	/// has many
	pub m_matchpo:Option<Vec<MMatchpo>>,
	/// has many
	pub m_packageline:Option<Vec<MPackageline>>,
	/// has many
	pub m_rmaline:Option<Vec<MRmaline>>,
	/// has many
	pub m_transaction:Option<Vec<MTransaction>>,
	/// has many
	pub m_transactionallocation:Option<Vec<MTransactionallocation>>,
	/// has many
	pub t_transaction:Option<Vec<TTransaction>>,
}

pub struct MInoutlineconfirm {
	/// primary
	/// not nullable 
	pub m_inoutlineconfirm_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub c_invoiceline_id:Option<f64>,
	pub confirmationno:Option<String>,
	/// defaults to: 0
	/// not nullable 
	pub confirmedqty:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub differenceqty:Option<f64>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub m_inoutconfirm_id:f64,
	/// not nullable 
	pub m_inoutline_id:f64,
	pub m_inventoryline_id:Option<f64>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	pub scrappedqty:Option<f64>,
	/// defaults to: 0
	/// not nullable 
	pub targetqty:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub m_inoutconfirm_id_m_inoutconfirm:Option<MInoutconfirm>,
	/// has one
	pub m_inoutline_id_m_inoutline:Option<MInoutline>,
	/// has one
	pub m_inventoryline_id_m_inventoryline:Option<MInventoryline>,
	/// has one
	pub c_invoiceline_id_c_invoiceline:Option<CInvoiceline>,
	/// has many
	pub i_inoutlineconfirm:Option<Vec<IInoutlineconfirm>>,
}

pub struct MInoutlinema {
	/// primary
	/// not nullable 
	pub m_inoutline_id:f64,
	/// primary
	/// not nullable 
	pub m_attributesetinstance_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 0
	/// not nullable 
	pub movementqty:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub m_inoutline_id_m_inoutline:Option<MInoutline>,
	/// has one
	pub m_attributesetinstance_id_m_attributesetinstance:Option<MAttributesetinstance>,
}

pub struct MInventory {
	/// primary
	/// not nullable 
	pub m_inventory_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_orgtrx_id:Option<f64>,
	pub approvalamt:Option<f64>,
	pub c_activity_id:Option<f64>,
	pub c_campaign_id:Option<f64>,
	/// not nullable 
	pub c_doctype_id:f64,
	pub c_project_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// not nullable 
	pub docaction:String,
	/// not nullable 
	pub docstatus:String,
	/// not nullable 
	pub documentno:String,
	/// defaults to: 'Y'::bpchar
	pub generatelist:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isapproved:String,
	pub m_perpetualinv_id:Option<f64>,
	/// not nullable 
	pub m_warehouse_id:f64,
	/// not nullable 
	pub movementdate:NaiveDateTime,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub posted:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	pub processing:Option<String>,
	pub reversal_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// defaults to: 'N'::bpchar
	pub updateqty:Option<String>,
	pub user1_id:Option<f64>,
	pub user2_id:Option<f64>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub m_warehouse_id_m_warehouse:Option<MWarehouse>,
	/// has one
	pub m_perpetualinv_id_m_perpetualinv:Option<MPerpetualinv>,
	/// has one
	pub ad_orgtrx_id_ad_org:Option<AdOrg>,
	/// has one
	pub c_project_id_c_project:Option<CProject>,
	/// has one
	pub c_campaign_id_c_campaign:Option<CCampaign>,
	/// has one
	pub c_activity_id_c_activity:Option<CActivity>,
	/// has one
	pub user1_id_c_elementvalue:Option<CElementvalue>,
	/// has one
	pub user2_id_c_elementvalue:Option<CElementvalue>,
	/// has one
	pub c_doctype_id_c_doctype:Option<CDoctype>,
	/// has one, self referential
	pub reversal_id_m_inventory:Option<Box<MInventory>>,
	/// has many
	pub i_inventory:Option<Vec<IInventory>>,
	/// has many
	pub m_inoutconfirm:Option<Vec<MInoutconfirm>>,
	/// has many
	pub m_inventory:Option<Vec<MInventory>>,
	/// has many
	pub m_inventoryline:Option<Vec<MInventoryline>>,
	/// has many
	pub m_movementconfirm:Option<Vec<MMovementconfirm>>,
}

pub struct MInventoryline {
	/// primary
	/// not nullable 
	pub m_inventoryline_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub c_charge_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'D'::bpchar
	/// not nullable 
	pub inventorytype:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub line:Option<f64>,
	/// defaults to: 0
	pub m_attributesetinstance_id:Option<f64>,
	/// not nullable 
	pub m_inventory_id:f64,
	/// not nullable 
	pub m_locator_id:f64,
	/// not nullable 
	pub m_product_id:f64,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	/// defaults to: 0
	/// not nullable 
	pub qtybook:f64,
	/// defaults to: 0
	/// not nullable 
	pub qtycount:f64,
	/// defaults to: 0
	/// not nullable 
	pub qtycsv:f64,
	pub qtyinternaluse:Option<f64>,
	pub reversalline_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub m_inventory_id_m_inventory:Option<MInventory>,
	/// has one
	pub m_locator_id_m_locator:Option<MLocator>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub m_attributesetinstance_id_m_attributesetinstance:Option<MAttributesetinstance>,
	/// has one
	pub c_charge_id_c_charge:Option<CCharge>,
	/// has many
	pub i_inventory:Option<Vec<IInventory>>,
	/// has many
	pub m_costdetail:Option<Vec<MCostdetail>>,
	/// has many
	pub m_inoutlineconfirm:Option<Vec<MInoutlineconfirm>>,
	/// has many
	pub m_inventorylinema:Option<Vec<MInventorylinema>>,
	/// has many
	pub m_movementlineconfirm:Option<Vec<MMovementlineconfirm>>,
	/// has many
	pub m_transaction:Option<Vec<MTransaction>>,
	/// has many
	pub m_transactionallocation:Option<Vec<MTransactionallocation>>,
	/// has many
	pub t_transaction:Option<Vec<TTransaction>>,
}

pub struct MInventorylinema {
	/// primary
	/// not nullable 
	pub m_inventoryline_id:f64,
	/// primary
	/// not nullable 
	pub m_attributesetinstance_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 0
	/// not nullable 
	pub movementqty:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub m_inventoryline_id_m_inventoryline:Option<MInventoryline>,
	/// has one
	pub m_attributesetinstance_id_m_attributesetinstance:Option<MAttributesetinstance>,
}

pub struct MLocator {
	/// primary
	/// not nullable 
	pub m_locator_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isdefault:String,
	/// not nullable 
	pub m_warehouse_id:f64,
	/// not nullable 
	pub priorityno:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
	pub x:Option<String>,
	pub y:Option<String>,
	pub z:Option<String>,
	/// has one
	pub ad_client_id_ad_client:Option<AdClient>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub m_warehouse_id_m_warehouse:Option<MWarehouse>,
	/// has many
	pub a_asset:Option<Vec<AAsset>>,
	/// has many
	pub c_projectissue:Option<Vec<CProjectissue>>,
	/// has many
	pub dd_orderline:Option<Vec<DdOrderline>>,
	/// has many
	pub fact_acct:Option<Vec<FactAcct>>,
	/// has many
	pub i_asset:Option<Vec<IAsset>>,
	/// has many
	pub i_inventory:Option<Vec<IInventory>>,
	/// has many
	pub m_inoutline:Option<Vec<MInoutline>>,
	/// has many
	pub m_inventoryline:Option<Vec<MInventoryline>>,
	/// has many
	pub m_movementline:Option<Vec<MMovementline>>,
	/// has many
	pub m_product:Option<Vec<MProduct>>,
	/// has many
	pub m_productionline:Option<Vec<MProductionline>>,
	/// has many
	pub m_productionplan:Option<Vec<MProductionplan>>,
	/// has many
	pub m_storage:Option<Vec<MStorage>>,
	/// has many
	pub m_transaction:Option<Vec<MTransaction>>,
	/// has many
	pub t_transaction:Option<Vec<TTransaction>>,
}

pub struct MLot {
	/// primary
	/// not nullable 
	pub m_lot_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub datefrom:Option<NaiveDateTime>,
	pub dateto:Option<NaiveDateTime>,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub m_lotctl_id:Option<f64>,
	/// not nullable 
	pub m_product_id:f64,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub m_lotctl_id_m_lotctl:Option<MLotctl>,
	/// has many
	pub m_attributesetinstance:Option<Vec<MAttributesetinstance>>,
}

pub struct MLotctl {
	/// primary
	/// not nullable 
	pub m_lotctl_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub currentnext:f64,
	pub description:Option<String>,
	/// not nullable 
	pub incrementno:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	pub prefix:Option<String>,
	/// not nullable 
	pub startno:f64,
	pub suffix:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has many
	pub m_attributeset:Option<Vec<MAttributeset>>,
	/// has many
	pub m_lot:Option<Vec<MLot>>,
	/// has many
	pub m_lotctlexclude:Option<Vec<MLotctlexclude>>,
}

pub struct MLotctlexclude {
	/// primary
	/// not nullable 
	pub m_lotctlexclude_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_table_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub issotrx:String,
	/// not nullable 
	pub m_lotctl_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub m_lotctl_id_m_lotctl:Option<MLotctl>,
	/// has one
	pub ad_table_id_ad_table:Option<AdTable>,
}

pub struct MMatchinv {
	/// primary
	/// not nullable 
	pub m_matchinv_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_invoiceline_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub dateacct:Option<NaiveDateTime>,
	/// not nullable 
	pub datetrx:NaiveDateTime,
	pub description:Option<String>,
	pub documentno:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub m_attributesetinstance_id:Option<f64>,
	/// not nullable 
	pub m_inoutline_id:f64,
	pub m_product_id:Option<f64>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub posted:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	/// not nullable 
	pub processing:String,
	/// defaults to: 0
	/// not nullable 
	pub qty:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub m_inoutline_id_m_inoutline:Option<MInoutline>,
	/// has one
	pub c_invoiceline_id_c_invoiceline:Option<CInvoiceline>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
}

pub struct MMatchpo {
	/// primary
	/// not nullable 
	pub m_matchpo_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub c_invoiceline_id:Option<f64>,
	/// not nullable 
	pub c_orderline_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub dateacct:Option<NaiveDateTime>,
	/// not nullable 
	pub datetrx:NaiveDateTime,
	pub description:Option<String>,
	pub documentno:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	pub isapproved:Option<String>,
	pub m_attributesetinstance_id:Option<f64>,
	pub m_inoutline_id:Option<f64>,
	pub m_product_id:Option<f64>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub posted:String,
	pub pricematchdifference:Option<f64>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	/// not nullable 
	pub processing:String,
	/// defaults to: 0
	/// not nullable 
	pub qty:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_orderline_id_c_orderline:Option<COrderline>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub m_inoutline_id_m_inoutline:Option<MInoutline>,
	/// has one
	pub c_invoiceline_id_c_invoiceline:Option<CInvoiceline>,
}

pub struct MMovement {
	/// primary
	/// not nullable 
	pub m_movement_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_orgtrx_id:Option<f64>,
	pub ad_user_id:Option<f64>,
	pub approvalamt:Option<f64>,
	pub c_activity_id:Option<f64>,
	pub c_bpartner_id:Option<f64>,
	pub c_bpartner_location_id:Option<f64>,
	pub c_campaign_id:Option<f64>,
	pub c_charge_id:Option<f64>,
	/// not nullable 
	pub c_doctype_id:f64,
	pub c_project_id:Option<f64>,
	pub chargeamt:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub createfrom:Option<String>,
	pub datereceived:Option<NaiveDateTime>,
	pub dd_order_id:Option<f64>,
	pub deliveryrule:Option<String>,
	pub deliveryviarule:Option<String>,
	pub description:Option<String>,
	/// not nullable 
	pub docaction:String,
	/// not nullable 
	pub docstatus:String,
	/// not nullable 
	pub documentno:String,
	pub freightamt:Option<f64>,
	pub freightcostrule:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isapproved:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isintransit:String,
	pub m_shipper_id:Option<f64>,
	/// not nullable 
	pub movementdate:NaiveDateTime,
	pub poreference:Option<String>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub posted:String,
	pub priorityrule:Option<String>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	pub processing:Option<String>,
	pub reversal_id:Option<f64>,
	pub salesrep_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub user1_id:Option<f64>,
	pub user2_id:Option<f64>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub ad_orgtrx_id_ad_org:Option<AdOrg>,
	/// has one
	pub c_project_id_c_project:Option<CProject>,
	/// has one
	pub c_campaign_id_c_campaign:Option<CCampaign>,
	/// has one
	pub c_activity_id_c_activity:Option<CActivity>,
	/// has one
	pub user1_id_c_elementvalue:Option<CElementvalue>,
	/// has one
	pub user2_id_c_elementvalue:Option<CElementvalue>,
	/// has one
	pub c_doctype_id_c_doctype:Option<CDoctype>,
	/// has one
	pub m_shipper_id_m_shipper:Option<MShipper>,
	/// has one
	pub salesrep_id_ad_user:Option<AdUser>,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub c_charge_id_c_charge:Option<CCharge>,
	/// has one
	pub dd_order_id_dd_order:Option<DdOrder>,
	/// has one, self referential
	pub reversal_id_m_movement:Option<Box<MMovement>>,
	/// has many
	pub m_movement:Option<Vec<MMovement>>,
	/// has many
	pub m_movementconfirm:Option<Vec<MMovementconfirm>>,
	/// has many
	pub m_movementline:Option<Vec<MMovementline>>,
}

pub struct MMovementconfirm {
	/// primary
	/// not nullable 
	pub m_movementconfirm_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: 0
	pub approvalamt:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// not nullable 
	pub docaction:String,
	/// not nullable 
	pub docstatus:String,
	/// not nullable 
	pub documentno:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isapproved:String,
	pub m_inventory_id:Option<f64>,
	/// not nullable 
	pub m_movement_id:f64,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	pub processing:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub m_movement_id_m_movement:Option<MMovement>,
	/// has one
	pub m_inventory_id_m_inventory:Option<MInventory>,
	/// has many
	pub m_movementlineconfirm:Option<Vec<MMovementlineconfirm>>,
}

pub struct MMovementline {
	/// primary
	/// not nullable 
	pub m_movementline_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: 0
	pub confirmedqty:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub dd_orderline_id:Option<f64>,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub line:Option<f64>,
	pub m_attributesetinstance_id:Option<f64>,
	pub m_attributesetinstanceto_id:Option<f64>,
	/// not nullable 
	pub m_locator_id:f64,
	/// not nullable 
	pub m_locatorto_id:f64,
	/// not nullable 
	pub m_movement_id:f64,
	/// not nullable 
	pub m_product_id:f64,
	/// defaults to: 0
	/// not nullable 
	pub movementqty:f64,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	pub reversalline_id:Option<f64>,
	/// defaults to: 0
	pub scrappedqty:Option<f64>,
	/// defaults to: 0
	pub targetqty:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub m_movement_id_m_movement:Option<MMovement>,
	/// has one
	pub m_locator_id_m_locator:Option<MLocator>,
	/// has one
	pub m_locatorto_id_m_locator:Option<MLocator>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub m_attributesetinstance_id_m_attributesetinstance:Option<MAttributesetinstance>,
	/// has one
	pub dd_orderline_id_dd_orderline:Option<DdOrderline>,
	/// has many
	pub m_costdetail:Option<Vec<MCostdetail>>,
	/// has many
	pub m_movementlineconfirm:Option<Vec<MMovementlineconfirm>>,
	/// has many
	pub m_movementlinema:Option<Vec<MMovementlinema>>,
	/// has many
	pub m_transaction:Option<Vec<MTransaction>>,
	/// has many
	pub t_transaction:Option<Vec<TTransaction>>,
}

pub struct MMovementlineconfirm {
	/// primary
	/// not nullable 
	pub m_movementlineconfirm_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: 0
	/// not nullable 
	pub confirmedqty:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 0
	/// not nullable 
	pub differenceqty:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub m_inventoryline_id:Option<f64>,
	/// not nullable 
	pub m_movementconfirm_id:f64,
	/// not nullable 
	pub m_movementline_id:f64,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	/// defaults to: 0
	/// not nullable 
	pub scrappedqty:f64,
	/// defaults to: 0
	/// not nullable 
	pub targetqty:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub m_movementconfirm_id_m_movementconfirm:Option<MMovementconfirm>,
	/// has one
	pub m_movementline_id_m_movementline:Option<MMovementline>,
	/// has one
	pub m_inventoryline_id_m_inventoryline:Option<MInventoryline>,
}

pub struct MMovementlinema {
	/// primary
	/// not nullable 
	pub m_movementline_id:f64,
	/// primary
	/// not nullable 
	pub m_attributesetinstance_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub movementqty:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub m_movementline_id_m_movementline:Option<MMovementline>,
	/// has one
	pub m_attributesetinstance_id_m_attributesetinstance:Option<MAttributesetinstance>,
}

pub struct MOperationresource {
	/// primary
	/// not nullable 
	pub m_operationresource_id:f64,
	pub a_asset_id:Option<f64>,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub c_job_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub m_productoperation_id:f64,
	/// not nullable 
	pub name:String,
	/// defaults to: 0
	/// not nullable 
	pub setuptime:f64,
	/// defaults to: 0
	/// not nullable 
	pub teardowntime:f64,
	/// defaults to: 0
	/// not nullable 
	pub unitruntime:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub m_productoperation_id_m_productoperation:Option<MProductoperation>,
	/// has one
	pub a_asset_id_a_asset:Option<AAsset>,
	/// has one
	pub c_job_id_c_job:Option<CJob>,
}

pub struct MPackage {
	/// primary
	/// not nullable 
	pub m_package_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub datereceived:Option<NaiveDateTime>,
	pub description:Option<String>,
	/// not nullable 
	pub documentno:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub m_inout_id:f64,
	/// not nullable 
	pub m_shipper_id:f64,
	pub receivedinfo:Option<String>,
	pub shipdate:Option<NaiveDateTime>,
	pub trackinginfo:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub m_inout_id_m_inout:Option<MInout>,
	/// has one
	pub m_shipper_id_m_shipper:Option<MShipper>,
	/// has many
	pub m_packageline:Option<Vec<MPackageline>>,
}

pub struct MPackageline {
	/// primary
	/// not nullable 
	pub m_packageline_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub m_inoutline_id:f64,
	/// not nullable 
	pub m_package_id:f64,
	/// defaults to: 0
	/// not nullable 
	pub qty:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub m_package_id_m_package:Option<MPackage>,
	/// has one
	pub m_inoutline_id_m_inoutline:Option<MInoutline>,
}

pub struct MPerpetualinv {
	/// primary
	/// not nullable 
	pub m_perpetualinv_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub counthighmovement:String,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub datelastrun:Option<NaiveDateTime>,
	/// not nullable 
	pub datenextrun:NaiveDateTime,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub m_product_category_id:Option<f64>,
	pub m_warehouse_id:Option<f64>,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub noinventorycount:f64,
	/// not nullable 
	pub noproductcount:f64,
	/// not nullable 
	pub numberofruns:f64,
	pub processing:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub m_product_category_id_m_product_category:Option<MProductCategory>,
	/// has one
	pub m_warehouse_id_m_warehouse:Option<MWarehouse>,
	/// has many
	pub m_inventory:Option<Vec<MInventory>>,
}

pub struct MPricelist {
	/// primary
	/// not nullable 
	pub m_pricelist_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub basepricelist_id:Option<f64>,
	/// not nullable 
	pub c_currency_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub enforcepricelimit:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isdefault:String,
	/// defaults to: 'N'::bpchar
	pub ismandatory:Option<String>,
	/// defaults to: 'N'::bpchar
	pub ispresentforproduct:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub issopricelist:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub istaxincluded:String,
	/// not nullable 
	pub name:String,
	/// defaults to: 2
	/// not nullable 
	pub priceprecision:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one, self referential
	pub basepricelist_id_m_pricelist:Option<Box<MPricelist>>,
	/// has one
	pub c_currency_id_c_currency:Option<CCurrency>,
	/// has many
	pub b_topictype:Option<Vec<BTopictype>>,
	/// has many
	pub c_bp_group:Option<Vec<CBpGroup>>,
	/// has many
	pub c_bpartner:Option<Vec<CBpartner>>,
	/// has many
	pub c_invoice:Option<Vec<CInvoice>>,
	/// has many
	pub c_order:Option<Vec<COrder>>,
	/// has many
	pub c_pos:Option<Vec<CPos>>,
	/// has many
	pub i_invoice:Option<Vec<IInvoice>>,
	/// has many
	pub i_order:Option<Vec<IOrder>>,
	/// has many
	pub m_pricelist:Option<Vec<MPricelist>>,
	/// has many
	pub m_pricelist_version:Option<Vec<MPricelistVersion>>,
	/// has many
	pub m_requisition:Option<Vec<MRequisition>>,
	/// has many
	pub s_timeexpense:Option<Vec<STimeexpense>>,
	/// has many
	pub w_basket:Option<Vec<WBasket>>,
	/// has many
	pub w_store:Option<Vec<WStore>>,
}

pub struct MPricelistVersion {
	/// primary
	/// not nullable 
	pub m_pricelist_version_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub m_discountschema_id:f64,
	/// not nullable 
	pub m_pricelist_id:f64,
	pub m_pricelist_version_base_id:Option<f64>,
	/// not nullable 
	pub name:String,
	pub proccreate:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub validfrom:NaiveDateTime,
	/// has one
	pub m_pricelist_id_m_pricelist:Option<MPricelist>,
	/// has one
	pub m_discountschema_id_m_discountschema:Option<MDiscountschema>,
	/// has one, self referential
	pub m_pricelist_version_base_id_m_pricelist_version:Option<Box<MPricelistVersion>>,
	/// has many
	pub c_project:Option<Vec<CProject>>,
	/// has many
	pub m_pricelist_version:Option<Vec<MPricelistVersion>>,
	/// has many
	pub m_productprice:Option<Vec<MProductprice>>,
	/// has many
	pub t_inventoryvalue:Option<Vec<TInventoryvalue>>,
}

pub struct MProduct {
	/// primary
	/// not nullable 
	pub m_product_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub c_revenuerecognition_id:Option<f64>,
	pub c_subscriptiontype_id:Option<f64>,
	/// not nullable 
	pub c_taxcategory_id:f64,
	/// not nullable 
	pub c_uom_id:f64,
	pub classification:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub descriptionurl:Option<String>,
	/// defaults to: 'N'::bpchar
	pub discontinued:Option<String>,
	pub discontinuedby:Option<NaiveDateTime>,
	pub documentnote:Option<String>,
	pub downloadurl:Option<String>,
	pub group1:Option<String>,
	pub group2:Option<String>,
	pub guaranteedays:Option<f64>,
	pub guaranteedaysmin:Option<f64>,
	pub help:Option<String>,
	pub imageurl:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isbom:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isdropship:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isexcludeautodelivery:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isinvoiceprintdetails:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub ispicklistprintdetails:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub ispurchased:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isselfservice:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub issold:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isstocked:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub issummary:String,
	pub istoformule:Option<String>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isverified:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub iswebstorefeatured:String,
	/// defaults to: (0)::numeric
	/// not nullable 
	pub lowlevel:f64,
	pub m_attributeset_id:Option<f64>,
	/// defaults to: 0
	pub m_attributesetinstance_id:Option<f64>,
	pub m_freightcategory_id:Option<f64>,
	pub m_locator_id:Option<f64>,
	/// not nullable 
	pub m_product_category_id:f64,
	/// not nullable 
	pub name:String,
	pub processing:Option<String>,
	/// defaults to: 'I'::bpchar
	/// not nullable 
	pub producttype:String,
	pub r_mailtext_id:Option<f64>,
	pub s_expensetype_id:Option<f64>,
	pub s_resource_id:Option<f64>,
	pub salesrep_id:Option<f64>,
	pub shelfdepth:Option<f64>,
	pub shelfheight:Option<f64>,
	pub shelfwidth:Option<f64>,
	pub sku:Option<String>,
	/// defaults to: 1
	/// not nullable 
	pub unitsperpack:f64,
	pub unitsperpallet:Option<f64>,
	pub upc:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
	pub versionno:Option<String>,
	/// defaults to: 0
	pub volume:Option<f64>,
	/// defaults to: 0
	pub weight:Option<f64>,
	/// has one
	pub ad_client_id_ad_client:Option<AdClient>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub c_uom_id_c_uom:Option<CUom>,
	/// has one
	pub salesrep_id_ad_user:Option<AdUser>,
	/// has one
	pub c_revenuerecognition_id_c_revenuerecognition:Option<CRevenuerecognition>,
	/// has one
	pub m_product_category_id_m_product_category:Option<MProductCategory>,
	/// has one
	pub c_taxcategory_id_c_taxcategory:Option<CTaxcategory>,
	/// has one
	pub s_resource_id_s_resource:Option<SResource>,
	/// has one
	pub s_expensetype_id_s_expensetype:Option<SExpensetype>,
	/// has one
	pub r_mailtext_id_r_mailtext:Option<RMailtext>,
	/// has one
	pub m_attributeset_id_m_attributeset:Option<MAttributeset>,
	/// has one
	pub m_attributesetinstance_id_m_attributesetinstance:Option<MAttributesetinstance>,
	/// has one
	pub m_freightcategory_id_m_freightcategory:Option<MFreightcategory>,
	/// has one
	pub m_locator_id_m_locator:Option<MLocator>,
	/// has one
	pub c_subscriptiontype_id_c_subscriptiontype:Option<CSubscriptiontype>,
	/// has many
	pub a_asset:Option<Vec<AAsset>>,
	/// has many
	pub a_registration:Option<Vec<ARegistration>>,
	/// has many
	pub a_registrationproduct:Option<Vec<ARegistrationproduct>>,
	/// has many
	pub ad_clientinfo:Option<Vec<AdClientinfo>>,
	/// has many
	pub b_topictype:Option<Vec<BTopictype>>,
	/// has many
	pub c_acctschema_element:Option<Vec<CAcctschemaElement>>,
	/// has many
	pub c_bpartner_product:Option<Vec<CBpartnerProduct>>,
	/// has many
	pub c_commissionline:Option<Vec<CCommissionline>>,
	/// has many
	pub c_invoiceline:Option<Vec<CInvoiceline>>,
	/// has many
	pub c_landedcost:Option<Vec<CLandedcost>>,
	/// has many
	pub c_landedcostallocation:Option<Vec<CLandedcostallocation>>,
	/// has many
	pub c_orderline:Option<Vec<COrderline>>,
	/// has many
	pub c_phase:Option<Vec<CPhase>>,
	/// has many
	pub c_poskey:Option<Vec<CPoskey>>,
	/// has many
	pub c_projectissue:Option<Vec<CProjectissue>>,
	/// has many
	pub c_projectline:Option<Vec<CProjectline>>,
	/// has many
	pub c_projectphase:Option<Vec<CProjectphase>>,
	/// has many
	pub c_projecttask:Option<Vec<CProjecttask>>,
	/// has many
	pub c_rfq_topicsubscriberonly:Option<Vec<CRfqTopicsubscriberonly>>,
	/// has many
	pub c_rfqline:Option<Vec<CRfqline>>,
	/// has many
	pub c_servicelevel:Option<Vec<CServicelevel>>,
	/// has many
	pub c_subscription:Option<Vec<CSubscription>>,
	/// has many
	pub c_task:Option<Vec<CTask>>,
	/// has many
	pub c_taxdefinition:Option<Vec<CTaxdefinition>>,
	/// has many
	pub c_uom_conversion:Option<Vec<CUomConversion>>,
	/// has many
	pub c_validcombination:Option<Vec<CValidcombination>>,
	/// has many
	pub dd_orderline:Option<Vec<DdOrderline>>,
	/// has many
	pub fact_acct:Option<Vec<FactAcct>>,
	/// has many
	pub gl_distribution:Option<Vec<GlDistribution>>,
	/// has many
	pub gl_distributionline:Option<Vec<GlDistributionline>>,
	/// has many
	pub i_asset:Option<Vec<IAsset>>,
	/// has many
	pub i_fajournal:Option<Vec<IFajournal>>,
	/// has many
	pub i_gljournal:Option<Vec<IGljournal>>,
	/// has many
	pub i_inventory:Option<Vec<IInventory>>,
	/// has many
	pub i_invoice:Option<Vec<IInvoice>>,
	/// has many
	pub i_order:Option<Vec<IOrder>>,
	/// has many
	pub i_product:Option<Vec<IProduct>>,
	/// has many
	pub m_bom:Option<Vec<MBom>>,
	/// has many
	pub m_bomalternative:Option<Vec<MBomalternative>>,
	/// has many
	pub m_bomproduct:Option<Vec<MBomproduct>>,
	/// has many
	pub m_cost:Option<Vec<MCost>>,
	/// has many
	pub m_costdetail:Option<Vec<MCostdetail>>,
	/// has many
	pub m_costqueue:Option<Vec<MCostqueue>>,
	/// has many
	pub m_demandline:Option<Vec<MDemandline>>,
	/// has many
	pub m_discountschemabreak:Option<Vec<MDiscountschemabreak>>,
	/// has many
	pub m_discountschemaline:Option<Vec<MDiscountschemaline>>,
	/// has many
	pub m_distributionrunline:Option<Vec<MDistributionrunline>>,
	/// has many
	pub m_edi:Option<Vec<MEdi>>,
	/// has many
	pub m_forecastline:Option<Vec<MForecastline>>,
	/// has many
	pub m_inoutline:Option<Vec<MInoutline>>,
	/// has many
	pub m_inventoryline:Option<Vec<MInventoryline>>,
	/// has many
	pub m_lot:Option<Vec<MLot>>,
	/// has many
	pub m_matchinv:Option<Vec<MMatchinv>>,
	/// has many
	pub m_matchpo:Option<Vec<MMatchpo>>,
	/// has many
	pub m_movementline:Option<Vec<MMovementline>>,
	/// has many
	pub m_product_acct:Option<Vec<MProductAcct>>,
	/// has many
	pub m_product_bom:Option<Vec<MProductBom>>,
	/// has many
	pub m_product_costing:Option<Vec<MProductCosting>>,
	/// has many
	pub m_product_po:Option<Vec<MProductPo>>,
	/// has many
	pub m_product_trl:Option<Vec<MProductTrl>>,
	/// has many
	pub m_productdownload:Option<Vec<MProductdownload>>,
	/// has many
	pub m_productionline:Option<Vec<MProductionline>>,
	/// has many
	pub m_productionplan:Option<Vec<MProductionplan>>,
	/// has many
	pub m_productoperation:Option<Vec<MProductoperation>>,
	/// has many
	pub m_productprice:Option<Vec<MProductprice>>,
	/// has many
	pub m_relatedproduct:Option<Vec<MRelatedproduct>>,
	/// has many
	pub m_replenish:Option<Vec<MReplenish>>,
	/// has many
	pub m_requisitionline:Option<Vec<MRequisitionline>>,
	/// has many
	pub m_storage:Option<Vec<MStorage>>,
	/// has many
	pub m_substitute:Option<Vec<MSubstitute>>,
	/// has many
	pub m_transaction:Option<Vec<MTransaction>>,
	/// has many
	pub m_transactionallocation:Option<Vec<MTransactionallocation>>,
	/// has many
	pub pa_goalrestriction:Option<Vec<PaGoalrestriction>>,
	/// has many
	pub pa_reportcolumn:Option<Vec<PaReportcolumn>>,
	/// has many
	pub pa_reportsource:Option<Vec<PaReportsource>>,
	/// has many
	pub pp_cost_collector:Option<Vec<PpCostCollector>>,
	/// has many
	pub pp_mrp:Option<Vec<PpMrp>>,
	/// has many
	pub pp_order:Option<Vec<PpOrder>>,
	/// has many
	pub pp_order_bom:Option<Vec<PpOrderBom>>,
	/// has many
	pub pp_order_bomline:Option<Vec<PpOrderBomline>>,
	/// has many
	pub pp_order_cost:Option<Vec<PpOrderCost>>,
	/// has many
	pub pp_order_node_product:Option<Vec<PpOrderNodeProduct>>,
	/// has many
	pub pp_product_bom:Option<Vec<PpProductBom>>,
	/// has many
	pub pp_product_bomline:Option<Vec<PpProductBomline>>,
	/// has many
	pub pp_product_planning:Option<Vec<PpProductPlanning>>,
	/// has many
	pub pp_wf_node_product:Option<Vec<PpWfNodeProduct>>,
	/// has many
	pub qm_specification:Option<Vec<QmSpecification>>,
	/// has many
	pub r_category:Option<Vec<RCategory>>,
	/// has many
	pub r_request:Option<Vec<RRequest>>,
	/// has many
	pub r_requestaction:Option<Vec<RRequestaction>>,
	/// has many
	pub r_requestupdate:Option<Vec<RRequestupdate>>,
	/// has many
	pub s_timeexpenseline:Option<Vec<STimeexpenseline>>,
	/// has many
	pub s_training_class:Option<Vec<STrainingClass>>,
	/// has many
	pub t_distributionrundetail:Option<Vec<TDistributionrundetail>>,
	/// has many
	pub t_inventoryvalue:Option<Vec<TInventoryvalue>>,
	/// has many
	pub t_replenish:Option<Vec<TReplenish>>,
	/// has many
	pub t_transaction:Option<Vec<TTransaction>>,
	/// has many
	pub w_basketline:Option<Vec<WBasketline>>,
}

pub struct MProductAcct {
	/// primary
	/// not nullable 
	pub m_product_id:f64,
	/// primary
	/// not nullable 
	pub c_acctschema_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub p_asset_acct:f64,
	pub p_burden_acct:Option<f64>,
	/// not nullable 
	pub p_cogs_acct:f64,
	pub p_costadjustment_acct:Option<f64>,
	pub p_costofproduction_acct:Option<f64>,
	/// not nullable 
	pub p_expense_acct:f64,
	pub p_floorstock_acct:Option<f64>,
	pub p_inventoryclearing_acct:Option<f64>,
	/// not nullable 
	pub p_invoicepricevariance_acct:f64,
	pub p_labor_acct:Option<f64>,
	pub p_methodchangevariance_acct:Option<f64>,
	pub p_mixvariance_acct:Option<f64>,
	pub p_outsideprocessing_acct:Option<f64>,
	pub p_overhead_acct:Option<f64>,
	/// not nullable 
	pub p_purchasepricevariance_acct:f64,
	pub p_ratevariance_acct:Option<f64>,
	/// not nullable 
	pub p_revenue_acct:f64,
	pub p_scrap_acct:Option<f64>,
	/// not nullable 
	pub p_tradediscountgrant_acct:f64,
	/// not nullable 
	pub p_tradediscountrec_acct:f64,
	pub p_usagevariance_acct:Option<f64>,
	pub p_wip_acct:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub c_acctschema_id_c_acctschema:Option<CAcctschema>,
	/// has one
	pub p_revenue_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub p_expense_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub p_asset_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub p_purchasepricevariance_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub p_invoicepricevariance_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub p_cogs_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub p_tradediscountrec_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub p_tradediscountgrant_acct_c_validcombination:Option<CValidcombination>,
}

pub struct MProductBom {
	/// primary
	/// not nullable 
	pub m_product_bom_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: 0
	/// not nullable 
	pub bomqty:f64,
	pub bomtype:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub line:f64,
	/// not nullable 
	pub m_product_id:f64,
	/// not nullable 
	pub m_productbom_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub m_productbom_id_m_product:Option<MProduct>,
}

pub struct MProductCategory {
	/// primary
	/// not nullable 
	pub m_product_category_id:f64,
	pub a_asset_group_id:Option<f64>,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_printcolor_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isdefault:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isselfservice:String,
	pub m_product_category_parent_id:Option<f64>,
	/// defaults to: 'F'::bpchar
	/// not nullable 
	pub mmpolicy:String,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub plannedmargin:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
	/// has one
	pub a_asset_group_id_a_asset_group:Option<AAssetGroup>,
	/// has one
	pub ad_printcolor_id_ad_printcolor:Option<AdPrintcolor>,
	/// has one, self referential
	pub m_product_category_parent_id_m_product_category:Option<Box<MProductCategory>>,
	/// has many
	pub c_commissionline:Option<Vec<CCommissionline>>,
	/// has many
	pub c_projectline:Option<Vec<CProjectline>>,
	/// has many
	pub c_rfq_topicsubscriberonly:Option<Vec<CRfqTopicsubscriberonly>>,
	/// has many
	pub c_taxdefinition:Option<Vec<CTaxdefinition>>,
	/// has many
	pub i_product:Option<Vec<IProduct>>,
	/// has many
	pub m_discountschemabreak:Option<Vec<MDiscountschemabreak>>,
	/// has many
	pub m_discountschemaline:Option<Vec<MDiscountschemaline>>,
	/// has many
	pub m_perpetualinv:Option<Vec<MPerpetualinv>>,
	/// has many
	pub m_product:Option<Vec<MProduct>>,
	/// has many
	pub m_product_category:Option<Vec<MProductCategory>>,
	/// has many
	pub m_product_category_acct:Option<Vec<MProductCategoryAcct>>,
	/// has many
	pub pa_goalrestriction:Option<Vec<PaGoalrestriction>>,
	/// has many
	pub s_expensetype:Option<Vec<SExpensetype>>,
	/// has many
	pub s_resourcetype:Option<Vec<SResourcetype>>,
	/// has many
	pub s_training:Option<Vec<STraining>>,
}

pub struct MProductCategoryAcct {
	/// primary
	/// not nullable 
	pub m_product_category_id:f64,
	/// primary
	/// not nullable 
	pub c_acctschema_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub costinglevel:Option<String>,
	pub costingmethod:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub p_asset_acct:f64,
	pub p_burden_acct:Option<f64>,
	/// not nullable 
	pub p_cogs_acct:f64,
	pub p_costadjustment_acct:Option<f64>,
	pub p_costofproduction_acct:Option<f64>,
	/// not nullable 
	pub p_expense_acct:f64,
	pub p_floorstock_acct:Option<f64>,
	pub p_inventoryclearing_acct:Option<f64>,
	/// not nullable 
	pub p_invoicepricevariance_acct:f64,
	pub p_labor_acct:Option<f64>,
	pub p_methodchangevariance_acct:Option<f64>,
	pub p_mixvariance_acct:Option<f64>,
	pub p_outsideprocessing_acct:Option<f64>,
	pub p_overhead_acct:Option<f64>,
	/// not nullable 
	pub p_purchasepricevariance_acct:f64,
	pub p_ratevariance_acct:Option<f64>,
	/// not nullable 
	pub p_revenue_acct:f64,
	pub p_scrap_acct:Option<f64>,
	/// not nullable 
	pub p_tradediscountgrant_acct:f64,
	/// not nullable 
	pub p_tradediscountrec_acct:f64,
	pub p_usagevariance_acct:Option<f64>,
	pub p_wip_acct:Option<f64>,
	pub processing:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub m_product_category_id_m_product_category:Option<MProductCategory>,
	/// has one
	pub c_acctschema_id_c_acctschema:Option<CAcctschema>,
	/// has one
	pub p_revenue_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub p_expense_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub p_asset_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub p_cogs_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub p_purchasepricevariance_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub p_invoicepricevariance_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub p_tradediscountrec_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub p_tradediscountgrant_acct_c_validcombination:Option<CValidcombination>,
}

pub struct MProductCosting {
	/// primary
	/// not nullable 
	pub m_product_id:f64,
	/// primary
	/// not nullable 
	pub c_acctschema_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: 0
	/// not nullable 
	pub costaverage:f64,
	/// defaults to: 0
	/// not nullable 
	pub costaveragecumamt:f64,
	/// defaults to: 0
	/// not nullable 
	pub costaveragecumqty:f64,
	/// defaults to: 0
	/// not nullable 
	pub coststandard:f64,
	/// defaults to: 0
	/// not nullable 
	pub coststandardcumamt:f64,
	/// defaults to: 0
	/// not nullable 
	pub coststandardcumqty:f64,
	/// defaults to: 0
	/// not nullable 
	pub coststandardpoamt:f64,
	/// defaults to: 0
	/// not nullable 
	pub coststandardpoqty:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 0
	/// not nullable 
	pub currentcostprice:f64,
	/// defaults to: 0
	/// not nullable 
	pub futurecostprice:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 0
	/// not nullable 
	pub pricelastinv:f64,
	/// defaults to: 0
	/// not nullable 
	pub pricelastpo:f64,
	/// defaults to: 0
	/// not nullable 
	pub totalinvamt:f64,
	/// defaults to: 0
	/// not nullable 
	pub totalinvqty:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub c_acctschema_id_c_acctschema:Option<CAcctschema>,
}

pub struct MProductPo {
	/// primary
	/// not nullable 
	pub m_product_id:f64,
	/// primary
	/// not nullable 
	pub c_bpartner_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub c_currency_id:Option<f64>,
	pub c_uom_id:Option<f64>,
	/// defaults to: 0
	pub costperorder:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub deliverytime_actual:Option<f64>,
	pub deliverytime_promised:Option<f64>,
	/// defaults to: 'N'::bpchar
	pub discontinued:Option<String>,
	pub discontinuedby:Option<NaiveDateTime>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub iscurrentvendor:String,
	pub manufacturer:Option<String>,
	/// defaults to: 0
	pub order_min:Option<f64>,
	/// defaults to: 0
	pub order_pack:Option<f64>,
	pub priceeffective:Option<NaiveDateTime>,
	/// defaults to: 0
	pub pricelastinv:Option<f64>,
	/// defaults to: 0
	pub pricelastpo:Option<f64>,
	/// defaults to: 0
	pub pricelist:Option<f64>,
	/// defaults to: 0
	pub pricepo:Option<f64>,
	pub qualityrating:Option<f64>,
	/// defaults to: 0
	pub royaltyamt:Option<f64>,
	pub upc:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub vendorcategory:Option<String>,
	/// not nullable 
	pub vendorproductno:String,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub ad_client_id_ad_client:Option<AdClient>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub c_uom_id_c_uom:Option<CUom>,
	/// has one
	pub c_currency_id_c_currency:Option<CCurrency>,
}

pub struct MProductTrl {
	/// primary
	/// not nullable 
	pub m_product_id:f64,
	/// primary
	/// not nullable 
	pub ad_language:String,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub documentnote:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub istranslated:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub ad_language_ad_language:Option<AdLanguage>,
}

pub struct MProductdownload {
	/// primary
	/// not nullable 
	pub m_productdownload_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub downloadurl:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub m_product_id:f64,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has many
	pub a_asset_delivery:Option<Vec<AAssetDelivery>>,
}

pub struct MProduction {
	/// primary
	/// not nullable 
	pub m_production_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_orgtrx_id:Option<f64>,
	pub c_activity_id:Option<f64>,
	pub c_campaign_id:Option<f64>,
	pub c_project_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub iscreated:String,
	/// not nullable 
	pub movementdate:NaiveDateTime,
	/// not nullable 
	pub name:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub posted:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	pub processing:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub user1_id:Option<f64>,
	pub user2_id:Option<f64>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub ad_orgtrx_id_ad_org:Option<AdOrg>,
	/// has one
	pub c_project_id_c_project:Option<CProject>,
	/// has one
	pub c_campaign_id_c_campaign:Option<CCampaign>,
	/// has one
	pub c_activity_id_c_activity:Option<CActivity>,
	/// has one
	pub user1_id_c_elementvalue:Option<CElementvalue>,
	/// has one
	pub user2_id_c_elementvalue:Option<CElementvalue>,
	/// has many
	pub m_productionplan:Option<Vec<MProductionplan>>,
}

pub struct MProductionline {
	/// primary
	/// not nullable 
	pub m_productionline_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub line:f64,
	/// defaults to: 0
	pub m_attributesetinstance_id:Option<f64>,
	/// not nullable 
	pub m_locator_id:f64,
	/// not nullable 
	pub m_product_id:f64,
	/// not nullable 
	pub m_productionplan_id:f64,
	/// defaults to: 0
	/// not nullable 
	pub movementqty:f64,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub m_productionplan_id_m_productionplan:Option<MProductionplan>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub m_locator_id_m_locator:Option<MLocator>,
	/// has one
	pub m_attributesetinstance_id_m_attributesetinstance:Option<MAttributesetinstance>,
	/// has many
	pub m_costdetail:Option<Vec<MCostdetail>>,
	/// has many
	pub m_productionlinema:Option<Vec<MProductionlinema>>,
	/// has many
	pub m_transaction:Option<Vec<MTransaction>>,
	/// has many
	pub m_transactionallocation:Option<Vec<MTransactionallocation>>,
	/// has many
	pub t_transaction:Option<Vec<TTransaction>>,
}

pub struct MProductionlinema {
	/// primary
	/// not nullable 
	pub m_productionline_id:f64,
	/// primary
	/// not nullable 
	pub m_attributesetinstance_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 0
	/// not nullable 
	pub movementqty:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub m_productionline_id_m_productionline:Option<MProductionline>,
	/// has one
	pub m_attributesetinstance_id_m_attributesetinstance:Option<MAttributesetinstance>,
}

pub struct MProductionplan {
	/// primary
	/// not nullable 
	pub m_productionplan_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub line:f64,
	/// not nullable 
	pub m_locator_id:f64,
	/// not nullable 
	pub m_product_id:f64,
	/// not nullable 
	pub m_production_id:f64,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	/// defaults to: 0
	/// not nullable 
	pub productionqty:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub m_production_id_m_production:Option<MProduction>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub m_locator_id_m_locator:Option<MLocator>,
	/// has many
	pub m_productionline:Option<Vec<MProductionline>>,
}

pub struct MProductoperation {
	/// primary
	/// not nullable 
	pub m_productoperation_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub m_product_id:f64,
	/// not nullable 
	pub name:String,
	/// defaults to: 0
	pub setuptime:Option<f64>,
	/// defaults to: 0
	pub teardowntime:Option<f64>,
	/// defaults to: 0
	pub unitruntime:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has many
	pub m_bomproduct:Option<Vec<MBomproduct>>,
	/// has many
	pub m_operationresource:Option<Vec<MOperationresource>>,
}

pub struct MProductprice {
	/// primary
	/// not nullable 
	pub m_pricelist_version_id:f64,
	/// primary
	/// not nullable 
	pub m_product_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 0
	/// not nullable 
	pub pricelimit:f64,
	/// defaults to: 0
	/// not nullable 
	pub pricelist:f64,
	/// defaults to: 0
	/// not nullable 
	pub pricestd:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub m_pricelist_version_id_m_pricelist_version:Option<MPricelistVersion>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
}

pub struct MProductpricevendorbreak {
	/// primary
	/// not nullable 
	pub m_productpricevendorbreak_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub breakvalue:f64,
	/// not nullable 
	pub c_bpartner_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub m_pricelist_version_id:f64,
	/// not nullable 
	pub m_product_id:f64,
	/// not nullable 
	pub pricelimit:f64,
	/// not nullable 
	pub pricelist:f64,
	/// not nullable 
	pub pricestd:f64,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
}

pub struct MPromotion {
	/// primary
	/// not nullable 
	pub m_promotion_id:f64,
	/// defaults to: NULL::numeric
	/// not nullable 
	pub ad_client_id:f64,
	/// defaults to: NULL::numeric
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: NULL::numeric
	pub c_campaign_id:Option<f64>,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// defaults to: 0
	/// not nullable 
	pub promotionpriority:f64,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
}

pub struct MPromotiondistribution {
	/// primary
	/// not nullable 
	pub m_promotiondistribution_id:f64,
	/// defaults to: NULL::numeric
	/// not nullable 
	pub ad_client_id:f64,
	/// defaults to: NULL::numeric
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub distributionsorting:Option<String>,
	/// not nullable 
	pub distributiontype:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub m_promotion_id:f64,
	/// not nullable 
	pub m_promotionline_id:f64,
	/// not nullable 
	pub operation:String,
	/// defaults to: 0
	/// not nullable 
	pub qty:f64,
	/// defaults to: NULL::numeric
	/// not nullable 
	pub seqno:f64,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
}

pub struct MPromotiongroup {
	/// primary
	/// not nullable 
	pub m_promotiongroup_id:f64,
	/// defaults to: NULL::numeric
	/// not nullable 
	pub ad_client_id:f64,
	/// defaults to: NULL::numeric
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
}

pub struct MPromotiongroupline {
	/// primary
	/// not nullable 
	pub m_promotiongroupline_id:f64,
	/// defaults to: NULL::numeric
	/// not nullable 
	pub ad_client_id:f64,
	/// defaults to: NULL::numeric
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub m_product_id:f64,
	/// not nullable 
	pub m_promotiongroup_id:f64,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
}

pub struct MPromotionline {
	/// primary
	/// not nullable 
	pub m_promotionline_id:f64,
	/// defaults to: NULL::numeric
	/// not nullable 
	pub ad_client_id:f64,
	/// defaults to: NULL::numeric
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub ismandatorypl:String,
	/// not nullable 
	pub m_promotion_id:f64,
	pub m_promotiongroup_id:Option<f64>,
	pub minimumamt:Option<f64>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
}

pub struct MPromotionprecondition {
	/// primary
	/// not nullable 
	pub m_promotionprecondition_id:f64,
	/// defaults to: NULL::numeric
	/// not nullable 
	pub ad_client_id:f64,
	/// defaults to: NULL::numeric
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: NULL::numeric
	pub c_activity_id:Option<f64>,
	pub c_bp_group_id:Option<f64>,
	pub c_bpartner_id:Option<f64>,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub enddate:Option<NaiveDateTime>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub m_pricelist_id:Option<f64>,
	/// not nullable 
	pub m_promotion_id:f64,
	pub m_warehouse_id:Option<f64>,
	pub promotioncode:Option<String>,
	/// defaults to: 0
	pub promotioncounter:Option<f64>,
	/// defaults to: 0
	pub promotionusagelimit:Option<f64>,
	/// defaults to: 0
	/// not nullable 
	pub seqno:f64,
	/// not nullable 
	pub startdate:NaiveDateTime,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
}

pub struct MPromotionreward {
	/// primary
	/// not nullable 
	pub m_promotionreward_id:f64,
	/// defaults to: NULL::numeric
	/// not nullable 
	pub ad_client_id:f64,
	/// defaults to: NULL::numeric
	/// not nullable 
	pub ad_org_id:f64,
	pub amount:Option<f64>,
	pub c_charge_id:Option<f64>,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub distributionsorting:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isforalldistribution:String,
	/// defaults to: 'Y'::bpchar
	pub issamedistribution:Option<String>,
	/// not nullable 
	pub m_promotion_id:f64,
	pub m_promotiondistribution_id:Option<f64>,
	pub m_targetdistribution_id:Option<f64>,
	pub qty:Option<f64>,
	/// not nullable 
	pub rewardtype:String,
	/// defaults to: NULL::numeric
	/// not nullable 
	pub seqno:f64,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
}

pub struct MRelatedproduct {
	/// primary
	/// not nullable 
	pub m_product_id:f64,
	/// primary
	/// not nullable 
	pub relatedproduct_id:f64,
	/// primary
	/// not nullable 
	pub relatedproducttype:String,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub relatedproduct_id_m_product:Option<MProduct>,
}

pub struct MReplenish {
	/// primary
	/// not nullable 
	pub m_product_id:f64,
	/// primary
	/// not nullable 
	pub m_warehouse_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 0
	/// not nullable 
	pub level_max:f64,
	/// defaults to: 0
	/// not nullable 
	pub level_min:f64,
	pub m_locator_id:Option<f64>,
	pub m_warehousesource_id:Option<f64>,
	/// not nullable 
	pub replenishtype:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub m_warehouse_id_m_warehouse:Option<MWarehouse>,
	/// has one
	pub m_warehousesource_id_m_warehouse:Option<MWarehouse>,
}

pub struct MRequisition {
	/// primary
	/// not nullable 
	pub m_requisition_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_user_id:f64,
	/// not nullable 
	pub c_doctype_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: now()
	/// not nullable 
	pub datedoc:NaiveDateTime,
	/// not nullable 
	pub daterequired:NaiveDateTime,
	pub description:Option<String>,
	/// not nullable 
	pub docaction:String,
	/// not nullable 
	pub docstatus:String,
	/// not nullable 
	pub documentno:String,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isapproved:String,
	/// not nullable 
	pub m_pricelist_id:f64,
	/// not nullable 
	pub m_warehouse_id:f64,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub posted:String,
	/// not nullable 
	pub priorityrule:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	pub processing:Option<String>,
	/// defaults to: 0
	/// not nullable 
	pub totallines:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
	/// has one
	pub m_pricelist_id_m_pricelist:Option<MPricelist>,
	/// has one
	pub m_warehouse_id_m_warehouse:Option<MWarehouse>,
	/// has one
	pub c_doctype_id_c_doctype:Option<CDoctype>,
	/// has many
	pub m_requisitionline:Option<Vec<MRequisitionline>>,
	/// has many
	pub pp_mrp:Option<Vec<PpMrp>>,
}

pub struct MRequisitionline {
	/// primary
	/// not nullable 
	pub m_requisitionline_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: NULL::numeric
	pub c_bpartner_id:Option<f64>,
	pub c_charge_id:Option<f64>,
	pub c_orderline_id:Option<f64>,
	/// defaults to: NULL::numeric
	pub c_uom_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub line:f64,
	/// defaults to: 0
	/// not nullable 
	pub linenetamt:f64,
	pub m_attributesetinstance_id:Option<f64>,
	pub m_product_id:Option<f64>,
	/// not nullable 
	pub m_requisition_id:f64,
	/// defaults to: 0
	/// not nullable 
	pub priceactual:f64,
	/// defaults to: 0
	/// not nullable 
	pub qty:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub m_requisition_id_m_requisition:Option<MRequisition>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub c_orderline_id_c_orderline:Option<COrderline>,
	/// has one
	pub c_charge_id_c_charge:Option<CCharge>,
	/// has many
	pub m_demanddetail:Option<Vec<MDemanddetail>>,
	/// has many
	pub pp_mrp:Option<Vec<PpMrp>>,
}

pub struct MRma {
	/// primary
	/// not nullable 
	pub m_rma_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub amt:Option<f64>,
	pub c_bpartner_id:Option<f64>,
	pub c_currency_id:Option<f64>,
	/// not nullable 
	pub c_doctype_id:f64,
	pub c_order_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// not nullable 
	pub docaction:String,
	/// not nullable 
	pub docstatus:String,
	/// not nullable 
	pub documentno:String,
	pub generateto:Option<String>,
	pub help:Option<String>,
	/// not nullable 
	pub inout_id:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isapproved:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub issotrx:String,
	pub m_rmatype_id:Option<f64>,
	/// not nullable 
	pub name:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	pub processing:Option<String>,
	/// defaults to: NULL::numeric
	pub ref_rma_id:Option<f64>,
	/// not nullable 
	pub salesrep_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub inout_id_m_inout:Option<MInout>,
	/// has one
	pub c_order_id_c_order:Option<COrder>,
	/// has one
	pub c_doctype_id_c_doctype:Option<CDoctype>,
	/// has one
	pub salesrep_id_ad_user:Option<AdUser>,
	/// has one
	pub m_rmatype_id_m_rmatype:Option<MRmatype>,
	/// has one
	pub c_currency_id_c_currency:Option<CCurrency>,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has many
	pub c_invoice:Option<Vec<CInvoice>>,
	/// has many
	pub m_inout:Option<Vec<MInout>>,
	/// has many
	pub m_rmaline:Option<Vec<MRmaline>>,
	/// has many
	pub r_request:Option<Vec<RRequest>>,
	/// has many
	pub r_requestaction:Option<Vec<RRequestaction>>,
}

pub struct MRmaline {
	/// primary
	/// not nullable 
	pub m_rmaline_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub amt:Option<f64>,
	pub c_charge_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 0
	/// not nullable 
	pub line:f64,
	pub linenetamt:Option<f64>,
	pub m_inoutline_id:Option<f64>,
	/// not nullable 
	pub m_rma_id:f64,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	/// defaults to: 0
	/// not nullable 
	pub qty:f64,
	pub qtydelivered:Option<f64>,
	pub qtyinvoiced:Option<f64>,
	/// defaults to: NULL::numeric
	pub ref_rmaline_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub m_rma_id_m_rma:Option<MRma>,
	/// has one
	pub m_inoutline_id_m_inoutline:Option<MInoutline>,
	/// has one
	pub c_charge_id_c_charge:Option<CCharge>,
	/// has many
	pub c_invoiceline:Option<Vec<CInvoiceline>>,
	/// has many
	pub m_inoutline:Option<Vec<MInoutline>>,
}

pub struct MRmatype {
	/// primary
	/// not nullable 
	pub m_rmatype_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has many
	pub m_rma:Option<Vec<MRma>>,
}

pub struct MSernoctl {
	/// primary
	/// not nullable 
	pub m_sernoctl_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub currentnext:f64,
	pub description:Option<String>,
	/// not nullable 
	pub incrementno:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	pub prefix:Option<String>,
	/// not nullable 
	pub startno:f64,
	pub suffix:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has many
	pub m_attributeset:Option<Vec<MAttributeset>>,
	/// has many
	pub m_sernoctlexclude:Option<Vec<MSernoctlexclude>>,
}

pub struct MSernoctlexclude {
	/// primary
	/// not nullable 
	pub m_sernoctlexclude_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_table_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub issotrx:String,
	/// not nullable 
	pub m_sernoctl_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub m_sernoctl_id_m_sernoctl:Option<MSernoctl>,
	/// has one
	pub ad_table_id_ad_table:Option<AdTable>,
}

pub struct MShipper {
	/// primary
	/// not nullable 
	pub m_shipper_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub c_bpartner_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	pub trackingurl:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has many
	pub c_order:Option<Vec<COrder>>,
	/// has many
	pub c_orderline:Option<Vec<COrderline>>,
	/// has many
	pub dd_networkdistributionline:Option<Vec<DdNetworkdistributionline>>,
	/// has many
	pub dd_order:Option<Vec<DdOrder>>,
	/// has many
	pub i_order:Option<Vec<IOrder>>,
	/// has many
	pub m_freight:Option<Vec<MFreight>>,
	/// has many
	pub m_inout:Option<Vec<MInout>>,
	/// has many
	pub m_movement:Option<Vec<MMovement>>,
	/// has many
	pub m_package:Option<Vec<MPackage>>,
}

pub struct MStorage {
	/// primary
	/// not nullable 
	pub m_product_id:f64,
	/// primary
	/// not nullable 
	pub m_locator_id:f64,
	/// primary
	/// defaults to: 0
	/// not nullable 
	pub m_attributesetinstance_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub datelastinventory:Option<NaiveDateTime>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 0
	/// not nullable 
	pub qtyonhand:f64,
	/// defaults to: 0
	/// not nullable 
	pub qtyordered:f64,
	/// defaults to: 0
	/// not nullable 
	pub qtyreserved:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub m_locator_id_m_locator:Option<MLocator>,
	/// has one
	pub ad_client_id_ad_client:Option<AdClient>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub m_attributesetinstance_id_m_attributesetinstance:Option<MAttributesetinstance>,
}

pub struct MSubstitute {
	/// primary
	/// not nullable 
	pub m_product_id:f64,
	/// primary
	/// not nullable 
	pub substitute_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub name:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub substitute_id_m_product:Option<MProduct>,
	/// has one
	pub ad_client_id_ad_client:Option<AdClient>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
}

pub struct MTransaction {
	/// primary
	/// not nullable 
	pub m_transaction_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub c_projectissue_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 0
	/// not nullable 
	pub m_attributesetinstance_id:f64,
	pub m_inoutline_id:Option<f64>,
	pub m_inventoryline_id:Option<f64>,
	/// not nullable 
	pub m_locator_id:f64,
	pub m_movementline_id:Option<f64>,
	/// not nullable 
	pub m_product_id:f64,
	pub m_productionline_id:Option<f64>,
	/// not nullable 
	pub movementdate:NaiveDateTime,
	/// defaults to: 0
	/// not nullable 
	pub movementqty:f64,
	/// not nullable 
	pub movementtype:String,
	pub pp_cost_collector_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub m_locator_id_m_locator:Option<MLocator>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub m_inventoryline_id_m_inventoryline:Option<MInventoryline>,
	/// has one
	pub m_movementline_id_m_movementline:Option<MMovementline>,
	/// has one
	pub m_inoutline_id_m_inoutline:Option<MInoutline>,
	/// has one
	pub m_productionline_id_m_productionline:Option<MProductionline>,
	/// has one
	pub c_projectissue_id_c_projectissue:Option<CProjectissue>,
	/// has one
	pub m_attributesetinstance_id_m_attributesetinstance:Option<MAttributesetinstance>,
	/// has many
	pub m_transactionallocation:Option<Vec<MTransactionallocation>>,
	/// has many
	pub t_transaction:Option<Vec<TTransaction>>,
}

pub struct MTransactionallocation {
	/// primary
	/// not nullable 
	pub m_transaction_id:f64,
	/// primary
	/// not nullable 
	pub allocationstrategytype:String,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isallocated:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub ismanual:String,
	/// defaults to: 0
	/// not nullable 
	pub m_attributesetinstance_id:f64,
	pub m_inoutline_id:Option<f64>,
	pub m_inventoryline_id:Option<f64>,
	/// not nullable 
	pub m_product_id:f64,
	pub m_productionline_id:Option<f64>,
	pub out_m_inoutline_id:Option<f64>,
	pub out_m_inventoryline_id:Option<f64>,
	pub out_m_productionline_id:Option<f64>,
	pub out_m_transaction_id:Option<f64>,
	/// defaults to: 0
	/// not nullable 
	pub qty:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub m_transaction_id_m_transaction:Option<MTransaction>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub m_attributesetinstance_id_m_attributesetinstance:Option<MAttributesetinstance>,
	/// has one
	pub m_inoutline_id_m_inoutline:Option<MInoutline>,
	/// has one
	pub m_productionline_id_m_productionline:Option<MProductionline>,
	/// has one
	pub m_inventoryline_id_m_inventoryline:Option<MInventoryline>,
	/// has one
	pub out_m_transaction_id_m_transaction:Option<MTransaction>,
	/// has one
	pub out_m_inoutline_id_m_inoutline:Option<MInoutline>,
	/// has one
	pub out_m_productionline_id_m_productionline:Option<MProductionline>,
	/// has one
	pub out_m_inventoryline_id_m_inventoryline:Option<MInventoryline>,
}

pub struct MWarehouse {
	/// primary
	/// not nullable 
	pub m_warehouse_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_location_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	pub isintransit:Option<String>,
	pub m_warehousesource_id:Option<f64>,
	/// not nullable 
	pub name:String,
	pub replenishmentclass:Option<String>,
	/// not nullable 
	pub separator:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
	/// has one
	pub ad_client_id_ad_client:Option<AdClient>,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub c_location_id_c_location:Option<CLocation>,
	/// has one, self referential
	pub m_warehousesource_id_m_warehouse:Option<Box<MWarehouse>>,
	/// has many
	pub ad_orginfo:Option<Vec<AdOrginfo>>,
	/// has many
	pub c_bp_edi:Option<Vec<CBpEdi>>,
	/// has many
	pub c_order:Option<Vec<COrder>>,
	/// has many
	pub c_orderline:Option<Vec<COrderline>>,
	/// has many
	pub c_pos:Option<Vec<CPos>>,
	/// has many
	pub c_project:Option<Vec<CProject>>,
	/// has many
	pub dd_networkdistributionline:Option<Vec<DdNetworkdistributionline>>,
	/// has many
	pub dd_order:Option<Vec<DdOrder>>,
	/// has many
	pub i_inventory:Option<Vec<IInventory>>,
	/// has many
	pub i_order:Option<Vec<IOrder>>,
	/// has many
	pub m_edi:Option<Vec<MEdi>>,
	/// has many
	pub m_forecastline:Option<Vec<MForecastline>>,
	/// has many
	pub m_inout:Option<Vec<MInout>>,
	/// has many
	pub m_inventory:Option<Vec<MInventory>>,
	/// has many
	pub m_locator:Option<Vec<MLocator>>,
	/// has many
	pub m_perpetualinv:Option<Vec<MPerpetualinv>>,
	/// has many
	pub m_replenish:Option<Vec<MReplenish>>,
	/// has many
	pub m_requisition:Option<Vec<MRequisition>>,
	/// has many
	pub m_warehouse:Option<Vec<MWarehouse>>,
	/// has many
	pub m_warehouse_acct:Option<Vec<MWarehouseAcct>>,
	/// has many
	pub pp_cost_collector:Option<Vec<PpCostCollector>>,
	/// has many
	pub pp_mrp:Option<Vec<PpMrp>>,
	/// has many
	pub pp_order:Option<Vec<PpOrder>>,
	/// has many
	pub pp_order_bomline:Option<Vec<PpOrderBomline>>,
	/// has many
	pub s_resource:Option<Vec<SResource>>,
	/// has many
	pub s_timeexpense:Option<Vec<STimeexpense>>,
	/// has many
	pub t_inventoryvalue:Option<Vec<TInventoryvalue>>,
	/// has many
	pub t_replenish:Option<Vec<TReplenish>>,
	/// has many
	pub w_store:Option<Vec<WStore>>,
}

pub struct MWarehouseAcct {
	/// primary
	/// not nullable 
	pub m_warehouse_id:f64,
	/// primary
	/// not nullable 
	pub c_acctschema_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub w_differences_acct:f64,
	/// not nullable 
	pub w_invactualadjust_acct:f64,
	/// not nullable 
	pub w_inventory_acct:f64,
	/// not nullable 
	pub w_revaluation_acct:f64,
	/// has one
	pub m_warehouse_id_m_warehouse:Option<MWarehouse>,
	/// has one
	pub c_acctschema_id_c_acctschema:Option<CAcctschema>,
	/// has one
	pub w_inventory_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub w_invactualadjust_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub w_differences_acct_c_validcombination:Option<CValidcombination>,
	/// has one
	pub w_revaluation_acct_c_validcombination:Option<CValidcombination>,
}

pub struct Name {
	/// primary
	/// not nullable 
	pub name_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: NULL::numeric
	pub datatype_id:Option<f64>,
	/// defaults to: NULL::character varying
	pub description:Option<String>,
	/// defaults to: NULL::character varying
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub length:Option<f64>,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
}

pub struct Object {
	/// primary
	/// not nullable 
	pub object_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: NULL::character varying
	pub description:Option<String>,
	/// defaults to: NULL::character varying
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// defaults to: (0)::numeric
	pub parent_id:Option<f64>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
}

pub struct ObjectAttribute {
	/// primary
	/// not nullable 
	pub object_attribute_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: NULL::numeric
	pub attribute_id:Option<f64>,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: NULL::numeric
	pub datatype_id:Option<f64>,
	/// defaults to: NULL::character varying
	pub description:Option<String>,
	/// defaults to: NULL::character varying
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: NULL::numeric
	pub object_id:Option<f64>,
	/// defaults to: NULL::numeric
	pub seqno:Option<f64>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
}

pub struct ObjectDiscriminatedAttribute {
	/// primary
	/// not nullable 
	pub object_discriminated_attribute:String,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: NULL::numeric
	pub attribute_id:Option<f64>,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: NULL::character varying
	pub description:Option<String>,
	/// defaults to: NULL::character varying
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: NULL::numeric
	pub object_id:Option<f64>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
}

pub struct ObjectObject {
	/// primary
	/// not nullable 
	pub object_object_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: NULL::character varying
	pub description:Option<String>,
	/// defaults to: NULL::numeric
	pub has_object_id:Option<f64>,
	/// defaults to: NULL::character varying
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: NULL::numeric
	pub object_id:Option<f64>,
	/// defaults to: NULL::numeric
	pub relationship_id:Option<f64>,
	/// defaults to: NULL::numeric
	pub seqno:Option<f64>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
}

pub struct ObjectView {
	/// primary
	/// not nullable 
	pub object_view_id:f64,
	/// defaults to: NULL::numeric
	pub ad_class_id:Option<f64>,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: NULL::numeric
	pub object_id:Option<f64>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// defaults to: NULL::numeric
	pub view_id:Option<f64>,
}

pub struct PaAchievement {
	/// primary
	/// not nullable 
	pub pa_achievement_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub datedoc:Option<NaiveDateTime>,
	pub description:Option<String>,
	/// not nullable 
	pub isachieved:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 0
	/// not nullable 
	pub manualactual:f64,
	/// not nullable 
	pub name:String,
	pub note:Option<String>,
	/// not nullable 
	pub pa_measure_id:f64,
	/// defaults to: 0
	/// not nullable 
	pub seqno:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub pa_measure_id_pa_measure:Option<PaMeasure>,
}

pub struct PaBenchmark {
	/// primary
	/// not nullable 
	pub pa_benchmark_id:f64,
	/// not nullable 
	pub accumulationtype:String,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has many
	pub pa_benchmarkdata:Option<Vec<PaBenchmarkdata>>,
	/// has many
	pub pa_measure:Option<Vec<PaMeasure>>,
}

pub struct PaBenchmarkdata {
	/// primary
	/// not nullable 
	pub pa_benchmarkdata_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub benchmarkdate:NaiveDateTime,
	/// not nullable 
	pub benchmarkvalue:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub pa_benchmark_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub pa_benchmark_id_pa_benchmark:Option<PaBenchmark>,
}

pub struct PaColorschema {
	/// primary
	/// not nullable 
	pub pa_colorschema_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_printcolor1_id:f64,
	/// not nullable 
	pub ad_printcolor2_id:f64,
	pub ad_printcolor3_id:Option<f64>,
	pub ad_printcolor4_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// not nullable 
	pub entitytype:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 0
	/// not nullable 
	pub mark1percent:f64,
	/// defaults to: 0
	/// not nullable 
	pub mark2percent:f64,
	/// defaults to: 0
	pub mark3percent:Option<f64>,
	/// defaults to: 0
	pub mark4percent:Option<f64>,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_printcolor1_id_ad_printcolor:Option<AdPrintcolor>,
	/// has one
	pub ad_printcolor2_id_ad_printcolor:Option<AdPrintcolor>,
	/// has one
	pub ad_printcolor3_id_ad_printcolor:Option<AdPrintcolor>,
	/// has one
	pub ad_printcolor4_id_ad_printcolor:Option<AdPrintcolor>,
	/// has one
	pub entitytype_ad_entitytype:Option<AdEntitytype>,
	/// has many
	pub pa_goal:Option<Vec<PaGoal>>,
}

pub struct PaDashboardcontent {
	/// primary
	/// not nullable 
	pub pa_dashboardcontent_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_window_id:Option<f64>,
	/// defaults to: (1)::numeric
	pub columnno:Option<f64>,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'T'::bpchar
	pub goaldisplay:Option<String>,
	pub html:Option<String>,
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub iscollapsible:String,
	pub line:Option<f64>,
	/// not nullable 
	pub name:String,
	pub pa_goal_id:Option<f64>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub zulfilepath:Option<String>,
	/// has one
	pub ad_window_id_ad_window:Option<AdWindow>,
	/// has one
	pub pa_goal_id_pa_goal:Option<PaGoal>,
}

pub struct PaGoal {
	/// primary
	/// not nullable 
	pub pa_goal_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_role_id:Option<f64>,
	pub ad_user_id:Option<f64>,
	/// defaults to: 'BC'::character varying
	/// not nullable 
	pub charttype:String,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub datefrom:Option<NaiveDateTime>,
	pub datelastrun:Option<NaiveDateTime>,
	pub dateto:Option<NaiveDateTime>,
	pub description:Option<String>,
	pub goalperformance:Option<f64>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub issummary:String,
	/// defaults to: 0
	pub measureactual:Option<f64>,
	pub measuredisplay:Option<String>,
	/// not nullable 
	pub measurescope:String,
	/// defaults to: 0
	/// not nullable 
	pub measuretarget:f64,
	/// not nullable 
	pub name:String,
	pub note:Option<String>,
	/// not nullable 
	pub pa_colorschema_id:f64,
	pub pa_goalparent_id:Option<f64>,
	pub pa_measure_id:Option<f64>,
	pub relativeweight:Option<f64>,
	/// defaults to: 0
	/// not nullable 
	pub seqno:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
	/// has one
	pub pa_colorschema_id_pa_colorschema:Option<PaColorschema>,
	/// has one, self referential
	pub pa_goalparent_id_pa_goal:Option<Box<PaGoal>>,
	/// has one
	pub pa_measure_id_pa_measure:Option<PaMeasure>,
	/// has one
	pub ad_role_id_ad_role:Option<AdRole>,
	/// has many
	pub pa_dashboardcontent:Option<Vec<PaDashboardcontent>>,
	/// has many
	pub pa_goal:Option<Vec<PaGoal>>,
	/// has many
	pub pa_goalrestriction:Option<Vec<PaGoalrestriction>>,
}

pub struct PaGoalrestriction {
	/// primary
	/// not nullable 
	pub pa_goalrestriction_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub c_bp_group_id:Option<f64>,
	pub c_bpartner_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub goalrestrictiontype:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub m_product_category_id:Option<f64>,
	pub m_product_id:Option<f64>,
	/// not nullable 
	pub name:String,
	pub org_id:Option<f64>,
	/// not nullable 
	pub pa_goal_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub pa_goal_id_pa_goal:Option<PaGoal>,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub org_id_ad_org:Option<AdOrg>,
	/// has one
	pub c_bp_group_id_c_bp_group:Option<CBpGroup>,
	/// has one
	pub m_product_category_id_m_product_category:Option<MProductCategory>,
}

pub struct PaHierarchy {
	/// primary
	/// not nullable 
	pub pa_hierarchy_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_tree_account_id:f64,
	/// not nullable 
	pub ad_tree_activity_id:f64,
	/// not nullable 
	pub ad_tree_bpartner_id:f64,
	/// not nullable 
	pub ad_tree_campaign_id:f64,
	/// not nullable 
	pub ad_tree_org_id:f64,
	/// not nullable 
	pub ad_tree_product_id:f64,
	/// not nullable 
	pub ad_tree_project_id:f64,
	/// not nullable 
	pub ad_tree_salesregion_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_tree_org_id_ad_tree:Option<AdTree>,
	/// has one
	pub ad_tree_bpartner_id_ad_tree:Option<AdTree>,
	/// has one
	pub ad_tree_project_id_ad_tree:Option<AdTree>,
	/// has one
	pub ad_tree_salesregion_id_ad_tree:Option<AdTree>,
	/// has one
	pub ad_tree_product_id_ad_tree:Option<AdTree>,
	/// has one
	pub ad_tree_campaign_id_ad_tree:Option<AdTree>,
	/// has one
	pub ad_tree_activity_id_ad_tree:Option<AdTree>,
	/// has one
	pub ad_tree_account_id_ad_tree:Option<AdTree>,
	/// has many
	pub pa_measure:Option<Vec<PaMeasure>>,
}

pub struct PaMeasure {
	/// primary
	/// not nullable 
	pub pa_measure_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub c_projecttype_id:Option<f64>,
	pub calculationclass:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub manualactual:Option<f64>,
	pub manualnote:Option<String>,
	/// not nullable 
	pub measuredatatype:String,
	/// not nullable 
	pub measuretype:String,
	/// not nullable 
	pub name:String,
	pub pa_benchmark_id:Option<f64>,
	pub pa_hierarchy_id:Option<f64>,
	pub pa_measurecalc_id:Option<f64>,
	pub pa_ratio_id:Option<f64>,
	pub r_requesttype_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub pa_measurecalc_id_pa_measurecalc:Option<PaMeasurecalc>,
	/// has one
	pub pa_benchmark_id_pa_benchmark:Option<PaBenchmark>,
	/// has one
	pub pa_ratio_id_pa_ratio:Option<PaRatio>,
	/// has one
	pub pa_hierarchy_id_pa_hierarchy:Option<PaHierarchy>,
	/// has one
	pub r_requesttype_id_r_requesttype:Option<RRequesttype>,
	/// has one
	pub c_projecttype_id_c_projecttype:Option<CProjecttype>,
	/// has many
	pub pa_achievement:Option<Vec<PaAchievement>>,
	/// has many
	pub pa_goal:Option<Vec<PaGoal>>,
}

pub struct PaMeasurecalc {
	/// primary
	/// not nullable 
	pub pa_measurecalc_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_table_id:f64,
	pub bpartnercolumn:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub datecolumn:String,
	pub description:Option<String>,
	/// not nullable 
	pub entitytype:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub keycolumn:String,
	/// not nullable 
	pub name:String,
	pub orgcolumn:Option<String>,
	pub productcolumn:Option<String>,
	/// not nullable 
	pub selectclause:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub whereclause:String,
	/// has one
	pub ad_table_id_ad_table:Option<AdTable>,
	/// has one
	pub entitytype_ad_entitytype:Option<AdEntitytype>,
	/// has many
	pub pa_measure:Option<Vec<PaMeasure>>,
	/// has many
	pub pa_ratioelement:Option<Vec<PaRatioelement>>,
}

pub struct PaRatio {
	/// primary
	/// not nullable 
	pub pa_ratio_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_acctschema_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_acctschema_id_c_acctschema:Option<CAcctschema>,
	/// has many
	pub pa_measure:Option<Vec<PaMeasure>>,
	/// has many
	pub pa_ratioelement:Option<Vec<PaRatioelement>>,
}

pub struct PaRatioelement {
	/// primary
	/// not nullable 
	pub pa_ratioelement_id:f64,
	pub account_id:Option<f64>,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub constantvalue:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	pub pa_measurecalc_id:Option<f64>,
	/// not nullable 
	pub pa_ratio_id:f64,
	pub pa_ratioused_id:Option<f64>,
	pub postingtype:Option<String>,
	/// not nullable 
	pub ratioelementtype:String,
	/// not nullable 
	pub ratiooperand:String,
	/// not nullable 
	pub seqno:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub pa_ratio_id_pa_ratio:Option<PaRatio>,
	/// has one
	pub account_id_c_elementvalue:Option<CElementvalue>,
	/// has one
	pub pa_ratioused_id_pa_ratio:Option<PaRatio>,
	/// has one
	pub pa_measurecalc_id_pa_measurecalc:Option<PaMeasurecalc>,
}

pub struct PaReport {
	/// primary
	/// not nullable 
	pub pa_report_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	pub ad_org_id:Option<f64>,
	pub ad_printformat_id:Option<f64>,
	/// not nullable 
	pub c_acctschema_id:f64,
	/// not nullable 
	pub c_calendar_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub jasperprocess_id:Option<f64>,
	pub jasperprocessing:Option<String>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub listsources:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub listtrx:String,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub pa_reportcolumnset_id:f64,
	/// defaults to: NULL::numeric
	pub pa_reportcube_id:Option<f64>,
	/// not nullable 
	pub pa_reportlineset_id:f64,
	/// not nullable 
	pub processing:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_org_id_ad_org:Option<AdOrg>,
	/// has one
	pub pa_reportlineset_id_pa_reportlineset:Option<PaReportlineset>,
	/// has one
	pub pa_reportcolumnset_id_pa_reportcolumnset:Option<PaReportcolumnset>,
	/// has one
	pub c_acctschema_id_c_acctschema:Option<CAcctschema>,
	/// has one
	pub c_calendar_id_c_calendar:Option<CCalendar>,
	/// has one
	pub ad_printformat_id_ad_printformat:Option<AdPrintformat>,
	/// has one
	pub jasperprocess_id_ad_process:Option<AdProcess>,
}

pub struct PaReportcolumn {
	/// primary
	/// not nullable 
	pub pa_reportcolumn_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_orgtrx_id:Option<f64>,
	pub amounttype:Option<String>,
	pub c_activity_id:Option<f64>,
	pub c_bpartner_id:Option<f64>,
	pub c_campaign_id:Option<f64>,
	pub c_currency_id:Option<f64>,
	pub c_elementvalue_id:Option<f64>,
	pub c_location_id:Option<f64>,
	pub c_project_id:Option<f64>,
	pub c_salesregion_id:Option<f64>,
	pub calculationtype:Option<String>,
	/// not nullable 
	pub columntype:String,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub currencytype:Option<String>,
	pub description:Option<String>,
	pub elementtype:Option<String>,
	pub factor:Option<String>,
	pub formatpattern:Option<String>,
	pub gl_budget_id:Option<f64>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	pub isadhocconversion:Option<String>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isincludenullsactivity:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isincludenullsbpartner:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isincludenullscampaign:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isincludenullselementvalue:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isincludenullslocation:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isincludenullsorg:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isincludenullsorgtrx:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isincludenullsproduct:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isincludenullsproject:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isincludenullssalesregion:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isincludenullsuserelement1:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isincludenullsuserelement2:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isprinted:String,
	pub m_product_id:Option<f64>,
	/// not nullable 
	pub name:String,
	pub oper_1_id:Option<f64>,
	pub oper_2_id:Option<f64>,
	pub org_id:Option<f64>,
	/// not nullable 
	pub pa_reportcolumnset_id:f64,
	/// not nullable 
	pub postingtype:String,
	pub relativeperiod:Option<f64>,
	/// not nullable 
	pub seqno:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub userelement1_id:Option<f64>,
	pub userelement2_id:Option<f64>,
	/// has one
	pub pa_reportcolumnset_id_pa_reportcolumnset:Option<PaReportcolumnset>,
	/// has one
	pub gl_budget_id_gl_budget:Option<GlBudget>,
	/// has one
	pub c_currency_id_c_currency:Option<CCurrency>,
	/// has one, self referential
	pub oper_1_id_pa_reportcolumn:Option<Box<PaReportcolumn>>,
	/// has one, self referential
	pub oper_2_id_pa_reportcolumn:Option<Box<PaReportcolumn>>,
	/// has one
	pub org_id_ad_org:Option<AdOrg>,
	/// has one
	pub c_elementvalue_id_c_elementvalue:Option<CElementvalue>,
	/// has one
	pub c_project_id_c_project:Option<CProject>,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub c_campaign_id_c_campaign:Option<CCampaign>,
	/// has one
	pub c_location_id_c_location:Option<CLocation>,
	/// has one
	pub c_salesregion_id_c_salesregion:Option<CSalesregion>,
	/// has one
	pub c_activity_id_c_activity:Option<CActivity>,
	/// has many
	pub pa_reportcolumn:Option<Vec<PaReportcolumn>>,
}

pub struct PaReportcolumnset {
	/// primary
	/// not nullable 
	pub pa_reportcolumnset_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub processing:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has many
	pub pa_report:Option<Vec<PaReport>>,
	/// has many
	pub pa_reportcolumn:Option<Vec<PaReportcolumn>>,
}

pub struct PaReportcube {
	/// primary
	/// not nullable 
	pub pa_reportcube_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_calendar_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: NULL::character varying
	pub description:Option<String>,
	/// defaults to: NULL::bpchar
	pub isactive:Option<String>,
	/// defaults to: NULL::bpchar
	pub isactivitydim:Option<String>,
	/// defaults to: NULL::bpchar
	pub isbpartnerdim:Option<String>,
	/// defaults to: NULL::bpchar
	pub iscampaigndim:Option<String>,
	/// defaults to: NULL::bpchar
	pub isglbudgetdim:Option<String>,
	/// defaults to: NULL::bpchar
	pub islocfromdim:Option<String>,
	/// defaults to: NULL::bpchar
	pub isloctodim:Option<String>,
	/// defaults to: NULL::bpchar
	pub isorgtrxdim:Option<String>,
	/// defaults to: NULL::bpchar
	pub isproductdim:Option<String>,
	/// defaults to: NULL::bpchar
	pub isprojectdim:Option<String>,
	/// defaults to: NULL::bpchar
	pub isprojectphasedim:Option<String>,
	/// defaults to: NULL::bpchar
	pub isprojecttaskdim:Option<String>,
	/// defaults to: NULL::bpchar
	pub issalesregiondim:Option<String>,
	/// defaults to: NULL::bpchar
	pub issubacctdim:Option<String>,
	/// defaults to: NULL::bpchar
	pub isuser1dim:Option<String>,
	/// defaults to: NULL::bpchar
	pub isuser2dim:Option<String>,
	/// defaults to: NULL::bpchar
	pub isuserelement1dim:Option<String>,
	/// defaults to: NULL::bpchar
	pub isuserelement2dim:Option<String>,
	pub lastrecalculated:Option<NaiveDateTime>,
	/// not nullable 
	pub name:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processing:String,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
}

pub struct PaReportline {
	/// primary
	/// not nullable 
	pub pa_reportline_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub amounttype:Option<String>,
	pub calculationtype:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub gl_budget_id:Option<f64>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isprinted:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub issummary:String,
	/// not nullable 
	pub linetype:String,
	/// not nullable 
	pub name:String,
	pub oper_1_id:Option<f64>,
	pub oper_2_id:Option<f64>,
	/// not nullable 
	pub pa_reportlineset_id:f64,
	pub parent_id:Option<f64>,
	pub postingtype:Option<String>,
	/// not nullable 
	pub seqno:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub pa_reportlineset_id_pa_reportlineset:Option<PaReportlineset>,
	/// has one, self referential
	pub parent_id_pa_reportline:Option<Box<PaReportline>>,
	/// has one, self referential
	pub oper_1_id_pa_reportline:Option<Box<PaReportline>>,
	/// has one, self referential
	pub oper_2_id_pa_reportline:Option<Box<PaReportline>>,
	/// has one
	pub gl_budget_id_gl_budget:Option<GlBudget>,
	/// has many
	pub i_reportline:Option<Vec<IReportline>>,
	/// has many
	pub pa_reportline:Option<Vec<PaReportline>>,
	/// has many
	pub pa_reportsource:Option<Vec<PaReportsource>>,
	/// has many
	pub t_report:Option<Vec<TReport>>,
}

pub struct PaReportlineset {
	/// primary
	/// not nullable 
	pub pa_reportlineset_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub processing:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has many
	pub i_reportline:Option<Vec<IReportline>>,
	/// has many
	pub pa_report:Option<Vec<PaReport>>,
	/// has many
	pub pa_reportline:Option<Vec<PaReportline>>,
}

pub struct PaReportsource {
	/// primary
	/// not nullable 
	pub pa_reportsource_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_orgtrx_id:Option<f64>,
	pub c_activity_id:Option<f64>,
	pub c_bpartner_id:Option<f64>,
	pub c_campaign_id:Option<f64>,
	pub c_elementvalue_id:Option<f64>,
	pub c_location_id:Option<f64>,
	pub c_project_id:Option<f64>,
	pub c_salesregion_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// not nullable 
	pub elementtype:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isincludenullsactivity:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isincludenullsbpartner:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isincludenullscampaign:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isincludenullselementvalue:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isincludenullslocation:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isincludenullsorg:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isincludenullsorgtrx:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isincludenullsproduct:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isincludenullsproject:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isincludenullssalesregion:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isincludenullsuserelement1:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isincludenullsuserelement2:String,
	pub m_product_id:Option<f64>,
	pub org_id:Option<f64>,
	/// not nullable 
	pub pa_reportline_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub userelement1_id:Option<f64>,
	pub userelement2_id:Option<f64>,
	/// has one
	pub pa_reportline_id_pa_reportline:Option<PaReportline>,
	/// has one
	pub org_id_ad_org:Option<AdOrg>,
	/// has one
	pub c_elementvalue_id_c_elementvalue:Option<CElementvalue>,
	/// has one
	pub c_project_id_c_project:Option<CProject>,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub c_campaign_id_c_campaign:Option<CCampaign>,
	/// has one
	pub c_location_id_c_location:Option<CLocation>,
	/// has one
	pub c_salesregion_id_c_salesregion:Option<CSalesregion>,
	/// has one
	pub c_activity_id_c_activity:Option<CActivity>,
	/// has many
	pub i_reportline:Option<Vec<IReportline>>,
}

pub struct PaSlaCriteria {
	/// primary
	/// not nullable 
	pub pa_sla_criteria_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub classname:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub ismanual:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has many
	pub pa_sla_goal:Option<Vec<PaSlaGoal>>,
}

pub struct PaSlaGoal {
	/// primary
	/// not nullable 
	pub pa_sla_goal_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_bpartner_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub datelastrun:Option<NaiveDateTime>,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 0
	/// not nullable 
	pub measureactual:f64,
	/// defaults to: 0
	/// not nullable 
	pub measuretarget:f64,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub pa_sla_criteria_id:f64,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	pub processing:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub validfrom:Option<NaiveDateTime>,
	pub validto:Option<NaiveDateTime>,
	/// has one
	pub pa_sla_criteria_id_pa_sla_criteria:Option<PaSlaCriteria>,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has many
	pub pa_sla_measure:Option<Vec<PaSlaMeasure>>,
}

pub struct PaSlaMeasure {
	/// primary
	/// not nullable 
	pub pa_sla_measure_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_table_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub datetrx:NaiveDateTime,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 0
	/// not nullable 
	pub measureactual:f64,
	/// not nullable 
	pub pa_sla_goal_id:f64,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	pub processing:Option<String>,
	pub record_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub pa_sla_goal_id_pa_sla_goal:Option<PaSlaGoal>,
	/// has one
	pub ad_table_id_ad_table:Option<AdTable>,
}

pub struct Person {
	/// primary
	/// not nullable 
	pub person_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: NULL::character varying
	pub description:Option<String>,
	/// defaults to: NULL::character varying
	pub firstname:Option<String>,
	/// defaults to: NULL::character varying
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: NULL::character varying
	pub lastname:Option<String>,
	/// defaults to: NULL::character varying
	pub middlename:Option<String>,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
}

pub struct PpCostCollector {
	/// primary
	/// not nullable 
	pub pp_cost_collector_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_orgtrx_id:Option<f64>,
	pub ad_user_id:Option<f64>,
	pub c_activity_id:Option<f64>,
	pub c_campaign_id:Option<f64>,
	/// not nullable 
	pub c_doctype_id:f64,
	/// not nullable 
	pub c_doctypetarget_id:f64,
	pub c_project_id:Option<f64>,
	pub c_uom_id:Option<f64>,
	/// not nullable 
	pub costcollectortype:String,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub dateacct:NaiveDateTime,
	pub description:Option<String>,
	/// defaults to: 'CO'::bpchar
	pub docaction:Option<String>,
	/// defaults to: 'DR'::bpchar
	pub docstatus:Option<String>,
	/// not nullable 
	pub documentno:String,
	pub durationreal:Option<f64>,
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	pub isbatchtime:Option<String>,
	pub issubcontracting:Option<String>,
	pub m_attributesetinstance_id:Option<f64>,
	/// not nullable 
	pub m_locator_id:f64,
	/// not nullable 
	pub m_product_id:f64,
	/// not nullable 
	pub m_warehouse_id:f64,
	/// not nullable 
	pub movementdate:NaiveDateTime,
	/// defaults to: (0)::numeric
	/// not nullable 
	pub movementqty:f64,
	/// not nullable 
	pub posted:String,
	pub pp_order_bomline_id:Option<f64>,
	/// not nullable 
	pub pp_order_id:f64,
	pub pp_order_node_id:Option<f64>,
	pub pp_order_workflow_id:Option<f64>,
	/// not nullable 
	pub processed:String,
	pub processing:Option<String>,
	/// defaults to: (0)::numeric
	pub qtyreject:Option<f64>,
	pub reversal_id:Option<f64>,
	/// not nullable 
	pub s_resource_id:f64,
	/// defaults to: (0)::numeric
	pub scrappedqty:Option<f64>,
	/// defaults to: (0)::numeric
	pub setuptimereal:Option<f64>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub user1_id:Option<f64>,
	pub user2_id:Option<f64>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub ad_orgtrx_id_ad_org:Option<AdOrg>,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
	/// has one
	pub c_activity_id_c_activity:Option<CActivity>,
	/// has one
	pub c_campaign_id_c_campaign:Option<CCampaign>,
	/// has one
	pub c_doctypetarget_id_c_doctype:Option<CDoctype>,
	/// has one
	pub c_doctype_id_c_doctype:Option<CDoctype>,
	/// has one
	pub c_project_id_c_project:Option<CProject>,
	/// has one
	pub c_uom_id_c_uom:Option<CUom>,
	/// has one
	pub m_warehouse_id_m_warehouse:Option<MWarehouse>,
	/// has one
	pub pp_order_bomline_id_pp_order_bomline:Option<PpOrderBomline>,
	/// has one
	pub pp_order_id_pp_order:Option<PpOrder>,
	/// has one
	pub pp_order_node_id_pp_order_node:Option<PpOrderNode>,
	/// has one
	pub pp_order_workflow_id_pp_order_workflow:Option<PpOrderWorkflow>,
	/// has one
	pub s_resource_id_s_resource:Option<SResource>,
	/// has one
	pub user1_id_ad_user:Option<AdUser>,
	/// has one
	pub user2_id_ad_user:Option<AdUser>,
	/// has one, self referential
	pub reversal_id_pp_cost_collector:Option<Box<PpCostCollector>>,
	/// has many
	pub hr_movement:Option<Vec<HrMovement>>,
	/// has many
	pub pp_cost_collector:Option<Vec<PpCostCollector>>,
	/// has many
	pub pp_cost_collectorma:Option<Vec<PpCostCollectorma>>,
}

pub struct PpCostCollectorma {
	/// primary
	/// not nullable 
	pub pp_cost_collectorma_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub m_attributesetinstance_id:f64,
	/// not nullable 
	pub movementqty:f64,
	/// not nullable 
	pub pp_cost_collector_id:f64,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub pp_cost_collector_id_pp_cost_collector:Option<PpCostCollector>,
}

pub struct PpMrp {
	/// primary
	/// not nullable 
	pub pp_mrp_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub c_bpartner_id:Option<f64>,
	pub c_order_id:Option<f64>,
	pub c_orderline_id:Option<f64>,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub dateconfirm:Option<NaiveDateTime>,
	pub datefinishschedule:Option<NaiveDateTime>,
	/// not nullable 
	pub dateordered:NaiveDateTime,
	/// not nullable 
	pub datepromised:NaiveDateTime,
	pub datesimulation:Option<NaiveDateTime>,
	pub datestart:Option<NaiveDateTime>,
	pub datestartschedule:Option<NaiveDateTime>,
	pub dd_order_id:Option<f64>,
	pub dd_orderline_id:Option<f64>,
	pub description:Option<String>,
	pub docstatus:Option<String>,
	/// not nullable 
	pub isactive:String,
	pub isavailable:Option<String>,
	pub m_forecast_id:Option<f64>,
	pub m_forecastline_id:Option<f64>,
	pub m_product_id:Option<f64>,
	pub m_requisition_id:Option<f64>,
	pub m_requisitionline_id:Option<f64>,
	/// not nullable 
	pub m_warehouse_id:f64,
	pub name:Option<String>,
	pub ordertype:Option<String>,
	pub planner_id:Option<f64>,
	pub pp_order_bomline_id:Option<f64>,
	pub pp_order_id:Option<f64>,
	pub priority:Option<String>,
	pub qty:Option<f64>,
	pub s_resource_id:Option<f64>,
	pub typemrp:Option<String>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
	pub version:Option<f64>,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub c_orderline_id_c_orderline:Option<COrderline>,
	/// has one
	pub c_order_id_c_order:Option<COrder>,
	/// has one
	pub m_forecastline_id_m_forecastline:Option<MForecastline>,
	/// has one
	pub m_forecast_id_m_forecast:Option<MForecast>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub m_requisitionline_id_m_requisitionline:Option<MRequisitionline>,
	/// has one
	pub m_requisition_id_m_requisition:Option<MRequisition>,
	/// has one
	pub m_warehouse_id_m_warehouse:Option<MWarehouse>,
	/// has one
	pub pp_order_bomline_id_pp_order_bomline:Option<PpOrderBomline>,
	/// has one
	pub pp_order_id_pp_order:Option<PpOrder>,
	/// has one
	pub planner_id_ad_user:Option<AdUser>,
	/// has one
	pub s_resource_id_s_resource:Option<SResource>,
	/// has one
	pub dd_order_id_dd_order:Option<DdOrder>,
	/// has one
	pub dd_orderline_id_dd_orderline:Option<DdOrderline>,
}

pub struct PpOrder {
	/// primary
	/// not nullable 
	pub pp_order_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_orgtrx_id:Option<f64>,
	/// not nullable 
	pub ad_workflow_id:f64,
	/// defaults to: (0)::numeric
	pub assay:Option<f64>,
	pub c_activity_id:Option<f64>,
	pub c_campaign_id:Option<f64>,
	/// defaults to: (0)::numeric
	pub c_doctype_id:Option<f64>,
	/// defaults to: (0)::numeric
	/// not nullable 
	pub c_doctypetarget_id:f64,
	pub c_orderline_id:Option<f64>,
	pub c_project_id:Option<f64>,
	/// not nullable 
	pub c_uom_id:f64,
	pub copyfrom:Option<String>,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub dateconfirm:Option<NaiveDateTime>,
	pub datedelivered:Option<NaiveDateTime>,
	pub datefinish:Option<NaiveDateTime>,
	pub datefinishschedule:Option<NaiveDateTime>,
	/// not nullable 
	pub dateordered:NaiveDateTime,
	/// not nullable 
	pub datepromised:NaiveDateTime,
	pub datestart:Option<NaiveDateTime>,
	/// not nullable 
	pub datestartschedule:NaiveDateTime,
	pub description:Option<String>,
	/// defaults to: '--'::bpchar
	/// not nullable 
	pub docaction:String,
	/// defaults to: 'DR'::bpchar
	/// not nullable 
	pub docstatus:String,
	/// not nullable 
	pub documentno:String,
	pub floatafter:Option<f64>,
	pub floatbefored:Option<f64>,
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isapproved:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isprinted:String,
	pub isqtypercentage:Option<String>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isselected:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub issotrx:String,
	/// not nullable 
	pub line:f64,
	pub lot:Option<String>,
	pub m_attributesetinstance_id:Option<f64>,
	/// not nullable 
	pub m_product_id:f64,
	/// not nullable 
	pub m_warehouse_id:f64,
	pub ordertype:Option<String>,
	pub planner_id:Option<f64>,
	pub posted:Option<String>,
	/// not nullable 
	pub pp_product_bom_id:f64,
	/// not nullable 
	pub priorityrule:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	pub processing:Option<String>,
	/// defaults to: (0)::numeric
	pub qtybatchs:Option<f64>,
	/// defaults to: (0)::numeric
	pub qtybatchsize:Option<f64>,
	/// defaults to: (0)::numeric
	/// not nullable 
	pub qtydelivered:f64,
	/// defaults to: (1)::numeric
	pub qtyentered:Option<f64>,
	/// defaults to: (1)::numeric
	/// not nullable 
	pub qtyordered:f64,
	/// defaults to: (0)::numeric
	/// not nullable 
	pub qtyreject:f64,
	pub qtyreserved:Option<f64>,
	/// defaults to: (0)::numeric
	/// not nullable 
	pub qtyscrap:f64,
	/// not nullable 
	pub s_resource_id:f64,
	/// defaults to: 'D'::character varying
	pub scheduletype:Option<String>,
	pub serno:Option<String>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub user1_id:Option<f64>,
	pub user2_id:Option<f64>,
	/// defaults to: (100)::numeric
	/// not nullable 
	pub yield_:f64,
	/// has one
	pub s_resource_id_s_resource:Option<SResource>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub m_warehouse_id_m_warehouse:Option<MWarehouse>,
	/// has one
	pub c_activity_id_c_activity:Option<CActivity>,
	/// has one
	pub c_campaign_id_c_campaign:Option<CCampaign>,
	/// has one
	pub c_doctypetarget_id_c_doctype:Option<CDoctype>,
	/// has one
	pub c_doctype_id_c_doctype:Option<CDoctype>,
	/// has one
	pub c_orderline_id_c_orderline:Option<COrderline>,
	/// has one
	pub c_project_id_c_project:Option<CProject>,
	/// has one
	pub c_uom_id_c_uom:Option<CUom>,
	/// has one
	pub pp_product_bom_id_pp_product_bom:Option<PpProductBom>,
	/// has one
	pub planner_id_ad_user:Option<AdUser>,
	/// has one
	pub user1_id_c_elementvalue:Option<CElementvalue>,
	/// has one
	pub user2_id_c_elementvalue:Option<CElementvalue>,
	/// has one
	pub ad_orgtrx_id_ad_org:Option<AdOrg>,
	/// has one
	pub ad_workflow_id_ad_workflow:Option<AdWorkflow>,
	/// has many
	pub pp_cost_collector:Option<Vec<PpCostCollector>>,
	/// has many
	pub pp_mrp:Option<Vec<PpMrp>>,
	/// has many
	pub pp_order_bom:Option<Vec<PpOrderBom>>,
	/// has many
	pub pp_order_bomline:Option<Vec<PpOrderBomline>>,
	/// has many
	pub pp_order_cost:Option<Vec<PpOrderCost>>,
	/// has many
	pub pp_order_node:Option<Vec<PpOrderNode>>,
	/// has many
	pub pp_order_node_asset:Option<Vec<PpOrderNodeAsset>>,
	/// has many
	pub pp_order_node_product:Option<Vec<PpOrderNodeProduct>>,
	/// has many
	pub pp_order_nodenext:Option<Vec<PpOrderNodenext>>,
	/// has many
	pub pp_order_workflow:Option<Vec<PpOrderWorkflow>>,
}

pub struct PpOrderBom {
	/// primary
	/// not nullable 
	pub pp_order_bom_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub bomtype:Option<String>,
	pub bomuse:Option<String>,
	/// not nullable 
	pub c_uom_id:f64,
	pub copyfrom:Option<String>,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub documentno:Option<String>,
	pub help:Option<String>,
	/// not nullable 
	pub isactive:String,
	pub m_attributesetinstance_id:Option<f64>,
	pub m_changenotice_id:Option<f64>,
	/// not nullable 
	pub m_product_id:f64,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub pp_order_id:f64,
	pub processing:Option<String>,
	pub revision:Option<String>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub validfrom:NaiveDateTime,
	pub validto:Option<NaiveDateTime>,
	/// not nullable 
	pub value:String,
	/// has one
	pub c_uom_id_c_uom:Option<CUom>,
	/// has one
	pub m_changenotice_id_m_changenotice:Option<MChangenotice>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub pp_order_id_pp_order:Option<PpOrder>,
	/// has many
	pub pp_order_bomline:Option<Vec<PpOrderBomline>>,
}

pub struct PpOrderBomTrl {
	/// primary
	/// not nullable 
	pub ad_language:String,
	/// primary
	/// not nullable 
	pub pp_order_bom_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: NULL::character varying
	pub description:Option<String>,
	/// defaults to: NULL::character varying
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub istranslated:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
}

pub struct PpOrderBomline {
	/// primary
	/// not nullable 
	pub pp_order_bomline_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_user_id:Option<f64>,
	pub assay:Option<f64>,
	pub backflushgroup:Option<String>,
	/// not nullable 
	pub c_uom_id:f64,
	/// defaults to: 'CO'::bpchar
	pub componenttype:Option<String>,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub datedelivered:Option<NaiveDateTime>,
	pub description:Option<String>,
	pub feature:Option<String>,
	pub forecast:Option<f64>,
	pub help:Option<String>,
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub iscritical:String,
	pub isqtypercentage:Option<String>,
	pub issuemethod:Option<String>,
	pub leadtimeoffset:Option<f64>,
	/// not nullable 
	pub line:f64,
	pub m_attributesetinstance_id:Option<f64>,
	pub m_changenotice_id:Option<f64>,
	pub m_locator_id:Option<f64>,
	/// not nullable 
	pub m_product_id:f64,
	/// not nullable 
	pub m_warehouse_id:f64,
	/// not nullable 
	pub pp_order_bom_id:f64,
	/// not nullable 
	pub pp_order_id:f64,
	/// not nullable 
	pub qtybatch:f64,
	/// not nullable 
	pub qtybom:f64,
	/// not nullable 
	pub qtydelivered:f64,
	/// defaults to: (1)::numeric
	pub qtyentered:Option<f64>,
	/// not nullable 
	pub qtypost:f64,
	/// not nullable 
	pub qtyreject:f64,
	/// not nullable 
	pub qtyrequiered:f64,
	/// not nullable 
	pub qtyreserved:f64,
	/// not nullable 
	pub qtyscrap:f64,
	pub scrap:Option<f64>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub validfrom:NaiveDateTime,
	pub validto:Option<NaiveDateTime>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub c_uom_id_c_uom:Option<CUom>,
	/// has one
	pub m_changenotice_id_m_changenotice:Option<MChangenotice>,
	/// has one
	pub m_warehouse_id_m_warehouse:Option<MWarehouse>,
	/// has one
	pub pp_order_bom_id_pp_order_bom:Option<PpOrderBom>,
	/// has one
	pub pp_order_id_pp_order:Option<PpOrder>,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
	/// has many
	pub pp_cost_collector:Option<Vec<PpCostCollector>>,
	/// has many
	pub pp_mrp:Option<Vec<PpMrp>>,
}

pub struct PpOrderBomlineTrl {
	/// primary
	/// not nullable 
	pub ad_language:String,
	/// primary
	/// not nullable 
	pub pp_order_bomline_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: NULL::character varying
	pub description:Option<String>,
	/// defaults to: NULL::character varying
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub istranslated:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
}

pub struct PpOrderCost {
	/// primary
	/// not nullable 
	pub pp_order_cost_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_workflow_id:Option<f64>,
	/// not nullable 
	pub c_acctschema_id:f64,
	/// defaults to: 'x'::bpchar
	pub costingmethod:Option<String>,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub cumulatedamt:Option<f64>,
	pub cumulatedamtpost:Option<f64>,
	pub cumulatedqty:Option<f64>,
	pub cumulatedqtypost:Option<f64>,
	pub currentcostprice:Option<f64>,
	pub currentcostpricell:Option<f64>,
	pub currentqty:Option<f64>,
	/// not nullable 
	pub isactive:String,
	pub m_attributesetinstance_id:Option<f64>,
	pub m_costelement_id:Option<f64>,
	/// not nullable 
	pub m_costtype_id:f64,
	/// not nullable 
	pub m_product_id:f64,
	/// not nullable 
	pub pp_order_id:f64,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_workflow_id_ad_workflow:Option<AdWorkflow>,
	/// has one
	pub c_acctschema_id_c_acctschema:Option<CAcctschema>,
	/// has one
	pub m_costelement_id_m_costelement:Option<MCostelement>,
	/// has one
	pub m_costtype_id_m_costtype:Option<MCosttype>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub pp_order_id_pp_order:Option<PpOrder>,
}

pub struct PpOrderNode {
	/// primary
	/// not nullable 
	pub pp_order_node_id:f64,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub action:String,
	/// not nullable 
	pub ad_client_id:f64,
	pub ad_column_id:Option<f64>,
	pub ad_form_id:Option<f64>,
	pub ad_image_id:Option<f64>,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_process_id:Option<f64>,
	pub ad_task_id:Option<f64>,
	pub ad_wf_block_id:Option<f64>,
	/// not nullable 
	pub ad_wf_node_id:f64,
	pub ad_wf_responsible_id:Option<f64>,
	pub ad_window_id:Option<f64>,
	/// not nullable 
	pub ad_workflow_id:f64,
	pub attributename:Option<String>,
	pub attributevalue:Option<String>,
	pub c_bpartner_id:Option<f64>,
	/// not nullable 
	pub cost:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub datefinish:Option<NaiveDateTime>,
	pub datefinishschedule:Option<NaiveDateTime>,
	pub datestart:Option<NaiveDateTime>,
	pub datestartschedule:Option<NaiveDateTime>,
	pub description:Option<String>,
	pub docaction:Option<String>,
	pub docstatus:Option<String>,
	/// defaults to: (0)::numeric
	pub duration:Option<f64>,
	pub durationreal:Option<f64>,
	pub durationrequiered:Option<f64>,
	/// defaults to: 'U'::character varying
	/// not nullable 
	pub entitytype:String,
	pub finishmode:Option<String>,
	pub help:Option<String>,
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub iscentrallymaintained:String,
	pub ismilestone:Option<String>,
	pub issubcontracting:Option<String>,
	/// defaults to: 'X'::bpchar
	/// not nullable 
	pub joinelement:String,
	/// not nullable 
	pub limit:f64,
	pub movingtime:Option<f64>,
	/// not nullable 
	pub name:String,
	pub overlapunits:Option<f64>,
	/// not nullable 
	pub pp_order_id:f64,
	/// not nullable 
	pub pp_order_workflow_id:f64,
	/// not nullable 
	pub priority:f64,
	pub qtydelivered:Option<f64>,
	pub qtyreject:Option<f64>,
	pub qtyrequiered:Option<f64>,
	pub qtyscrap:Option<f64>,
	pub queuingtime:Option<f64>,
	pub s_resource_id:Option<f64>,
	pub setuptime:Option<f64>,
	pub setuptimereal:Option<f64>,
	pub setuptimerequiered:Option<f64>,
	/// defaults to: 'X'::bpchar
	/// not nullable 
	pub splitelement:String,
	pub startmode:Option<String>,
	pub subflowexecution:Option<String>,
	pub unitscycles:Option<f64>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub validfrom:Option<NaiveDateTime>,
	pub validto:Option<NaiveDateTime>,
	/// not nullable 
	pub value:String,
	/// not nullable 
	pub waitingtime:f64,
	pub workflow_id:Option<f64>,
	/// not nullable 
	pub workingtime:f64,
	/// not nullable 
	pub xposition:f64,
	/// defaults to: (100)::numeric
	pub yield_:Option<f64>,
	/// not nullable 
	pub yposition:f64,
	/// has one
	pub ad_column_id_ad_column:Option<AdColumn>,
	/// has one
	pub ad_form_id_ad_form:Option<AdForm>,
	/// has one
	pub ad_image_id_ad_image:Option<AdImage>,
	/// has one
	pub ad_process_id_ad_process:Option<AdProcess>,
	/// has one
	pub ad_task_id_ad_task:Option<AdTask>,
	/// has one
	pub ad_wf_block_id_ad_wf_block:Option<AdWfBlock>,
	/// has one
	pub ad_wf_node_id_ad_wf_node:Option<AdWfNode>,
	/// has one
	pub ad_wf_responsible_id_ad_wf_responsible:Option<AdWfResponsible>,
	/// has one
	pub ad_window_id_ad_window:Option<AdWindow>,
	/// has one
	pub ad_workflow_id_ad_workflow:Option<AdWorkflow>,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub entitytype_ad_entitytype:Option<AdEntitytype>,
	/// has one
	pub pp_order_id_pp_order:Option<PpOrder>,
	/// has one
	pub pp_order_workflow_id_pp_order_workflow:Option<PpOrderWorkflow>,
	/// has one
	pub s_resource_id_s_resource:Option<SResource>,
	/// has one
	pub workflow_id_ad_workflow:Option<AdWorkflow>,
	/// has many
	pub pp_cost_collector:Option<Vec<PpCostCollector>>,
	/// has many
	pub pp_order_node_asset:Option<Vec<PpOrderNodeAsset>>,
	/// has many
	pub pp_order_node_product:Option<Vec<PpOrderNodeProduct>>,
	/// has many
	pub pp_order_nodenext:Option<Vec<PpOrderNodenext>>,
	/// has many
	pub pp_order_workflow:Option<Vec<PpOrderWorkflow>>,
}

pub struct PpOrderNodeAsset {
	/// primary
	/// not nullable 
	pub pp_order_node_asset_id:f64,
	/// not nullable 
	pub a_asset_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub pp_order_id:f64,
	/// not nullable 
	pub pp_order_node_id:f64,
	/// not nullable 
	pub pp_order_workflow_id:f64,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub a_asset_id_a_asset:Option<AAsset>,
	/// has one
	pub pp_order_id_pp_order:Option<PpOrder>,
	/// has one
	pub pp_order_node_id_pp_order_node:Option<PpOrderNode>,
	/// has one
	pub pp_order_workflow_id_pp_order_workflow:Option<PpOrderWorkflow>,
}

pub struct PpOrderNodeProduct {
	/// primary
	/// not nullable 
	pub pp_order_node_product_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub isactive:String,
	pub issubcontracting:Option<String>,
	/// not nullable 
	pub m_product_id:f64,
	/// not nullable 
	pub pp_order_id:f64,
	/// not nullable 
	pub pp_order_node_id:f64,
	/// not nullable 
	pub pp_order_workflow_id:f64,
	pub qty:Option<f64>,
	pub seqno:Option<f64>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub yield_:Option<f64>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub pp_order_id_pp_order:Option<PpOrder>,
	/// has one
	pub pp_order_node_id_pp_order_node:Option<PpOrderNode>,
	/// has one
	pub pp_order_workflow_id_pp_order_workflow:Option<PpOrderWorkflow>,
}

pub struct PpOrderNodeTrl {
	/// primary
	/// not nullable 
	pub ad_language:String,
	/// primary
	/// not nullable 
	pub pp_order_node_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: NULL::character varying
	pub description:Option<String>,
	/// defaults to: NULL::character varying
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub istranslated:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
}

pub struct PpOrderNodenext {
	/// primary
	/// not nullable 
	pub pp_order_nodenext_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_wf_next_id:Option<f64>,
	/// not nullable 
	pub ad_wf_node_id:f64,
	pub created:Option<NaiveDateTime>,
	pub createdby:Option<f64>,
	pub description:Option<String>,
	/// defaults to: 'U'::character varying
	/// not nullable 
	pub entitytype:String,
	/// not nullable 
	pub isactive:String,
	pub isstduserworkflow:Option<String>,
	/// not nullable 
	pub pp_order_id:f64,
	pub pp_order_next_id:Option<f64>,
	/// not nullable 
	pub pp_order_node_id:f64,
	/// defaults to: (10)::numeric
	/// not nullable 
	pub seqno:f64,
	pub transitioncode:Option<String>,
	pub updated:Option<NaiveDateTime>,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_wf_next_id_ad_wf_node:Option<AdWfNode>,
	/// has one
	pub ad_wf_node_id_ad_wf_node:Option<AdWfNode>,
	/// has one
	pub entitytype_ad_entitytype:Option<AdEntitytype>,
	/// has one
	pub pp_order_id_pp_order:Option<PpOrder>,
	/// has one
	pub pp_order_next_id_pp_order_node:Option<PpOrderNode>,
	/// has one
	pub pp_order_node_id_pp_order_node:Option<PpOrderNode>,
}

pub struct PpOrderWorkflow {
	/// primary
	/// not nullable 
	pub pp_order_workflow_id:f64,
	/// not nullable 
	pub accesslevel:String,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_table_id:Option<f64>,
	pub ad_wf_node_id:Option<f64>,
	pub ad_wf_responsible_id:Option<f64>,
	/// not nullable 
	pub ad_workflow_id:f64,
	pub ad_workflowprocessor_id:Option<f64>,
	/// not nullable 
	pub author:String,
	pub cost:Option<f64>,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub documentno:Option<String>,
	/// defaults to: (0)::numeric
	/// not nullable 
	pub duration:f64,
	/// defaults to: 'h'::bpchar
	/// not nullable 
	pub durationunit:String,
	/// defaults to: 'U'::character varying
	/// not nullable 
	pub entitytype:String,
	pub help:Option<String>,
	/// not nullable 
	pub isactive:String,
	pub isdefault:Option<String>,
	/// not nullable 
	pub limit:f64,
	pub movingtime:Option<f64>,
	/// not nullable 
	pub name:String,
	pub overlapunits:Option<f64>,
	/// not nullable 
	pub pp_order_id:f64,
	pub pp_order_node_id:Option<f64>,
	/// not nullable 
	pub priority:f64,
	pub processtype:Option<String>,
	/// defaults to: 'U'::bpchar
	/// not nullable 
	pub publishstatus:String,
	/// defaults to: (1)::numeric
	pub qtybatchsize:Option<f64>,
	pub queuingtime:Option<f64>,
	pub s_resource_id:Option<f64>,
	pub setuptime:Option<f64>,
	pub unitscycles:Option<f64>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub validateworkflow:Option<String>,
	pub validfrom:Option<NaiveDateTime>,
	pub validto:Option<NaiveDateTime>,
	pub value:Option<String>,
	/// not nullable 
	pub version:f64,
	/// not nullable 
	pub waitingtime:f64,
	/// defaults to: 'M'::bpchar
	pub workflowtype:Option<String>,
	pub workingtime:Option<f64>,
	/// defaults to: (100)::numeric
	pub yield_:Option<f64>,
	/// has one
	pub ad_table_id_ad_table:Option<AdTable>,
	/// has one
	pub ad_wf_node_id_ad_wf_node:Option<AdWfNode>,
	/// has one
	pub ad_wf_responsible_id_ad_wf_responsible:Option<AdWfResponsible>,
	/// has one
	pub ad_workflowprocessor_id_ad_workflowprocessor:Option<AdWorkflowprocessor>,
	/// has one
	pub ad_workflow_id_ad_workflow:Option<AdWorkflow>,
	/// has one
	pub entitytype_ad_entitytype:Option<AdEntitytype>,
	/// has one
	pub pp_order_id_pp_order:Option<PpOrder>,
	/// has one
	pub pp_order_node_id_pp_order_node:Option<PpOrderNode>,
	/// has one
	pub s_resource_id_s_resource:Option<SResource>,
	/// has many
	pub pp_cost_collector:Option<Vec<PpCostCollector>>,
	/// has many
	pub pp_order_node:Option<Vec<PpOrderNode>>,
	/// has many
	pub pp_order_node_asset:Option<Vec<PpOrderNodeAsset>>,
	/// has many
	pub pp_order_node_product:Option<Vec<PpOrderNodeProduct>>,
}

pub struct PpOrderWorkflowTrl {
	/// primary
	/// not nullable 
	pub ad_language:String,
	/// primary
	/// not nullable 
	pub pp_order_workflow_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: NULL::character varying
	pub description:Option<String>,
	/// defaults to: NULL::character varying
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub istranslated:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
}

pub struct PpProductBom {
	/// primary
	/// not nullable 
	pub pp_product_bom_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: 'A'::bpchar
	pub bomtype:Option<String>,
	/// defaults to: 'M'::bpchar
	pub bomuse:Option<String>,
	pub c_uom_id:Option<f64>,
	pub copyfrom:Option<String>,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub documentno:Option<String>,
	pub help:Option<String>,
	/// not nullable 
	pub isactive:String,
	pub m_attributesetinstance_id:Option<f64>,
	pub m_changenotice_id:Option<f64>,
	/// not nullable 
	pub m_product_id:f64,
	/// not nullable 
	pub name:String,
	pub processing:Option<String>,
	pub revision:Option<String>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub validfrom:NaiveDateTime,
	pub validto:Option<NaiveDateTime>,
	/// not nullable 
	pub value:String,
	/// has one
	pub m_changenotice_id_m_changenotice:Option<MChangenotice>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub c_uom_id_c_uom:Option<CUom>,
	/// has many
	pub pp_order:Option<Vec<PpOrder>>,
	/// has many
	pub pp_product_bomline:Option<Vec<PpProductBomline>>,
	/// has many
	pub qm_specification:Option<Vec<QmSpecification>>,
	/// has many
	pub r_group:Option<Vec<RGroup>>,
}

pub struct PpProductBomTrl {
	/// primary
	/// not nullable 
	pub ad_language:String,
	/// primary
	/// not nullable 
	pub pp_product_bom_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: NULL::character varying
	pub description:Option<String>,
	/// defaults to: NULL::character varying
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub istranslated:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
}

pub struct PpProductBomline {
	/// primary
	/// not nullable 
	pub pp_product_bomline_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub assay:Option<f64>,
	pub backflushgroup:Option<String>,
	pub c_uom_id:Option<f64>,
	/// defaults to: 'CO'::bpchar
	pub componenttype:Option<String>,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub feature:Option<String>,
	pub forecast:Option<f64>,
	pub help:Option<String>,
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	pub iscritical:Option<String>,
	pub isqtypercentage:Option<String>,
	/// defaults to: '1'::bpchar
	/// not nullable 
	pub issuemethod:String,
	pub leadtimeoffset:Option<f64>,
	/// not nullable 
	pub line:f64,
	pub m_attributesetinstance_id:Option<f64>,
	pub m_changenotice_id:Option<f64>,
	/// not nullable 
	pub m_product_id:f64,
	/// not nullable 
	pub pp_product_bom_id:f64,
	pub qtybatch:Option<f64>,
	pub qtybom:Option<f64>,
	pub scrap:Option<f64>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub validfrom:NaiveDateTime,
	pub validto:Option<NaiveDateTime>,
	/// has one
	pub c_uom_id_c_uom:Option<CUom>,
	/// has one
	pub m_changenotice_id_m_changenotice:Option<MChangenotice>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub pp_product_bom_id_pp_product_bom:Option<PpProductBom>,
}

pub struct PpProductBomlineTrl {
	/// primary
	/// not nullable 
	pub ad_language:String,
	/// primary
	/// not nullable 
	pub pp_product_bomline_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: NULL::character varying
	pub description:Option<String>,
	/// defaults to: NULL::character varying
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub istranslated:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
}

pub struct PpProductPlanning {
	/// primary
	/// not nullable 
	pub pp_product_planning_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// defaults to: (0)::numeric
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_workflow_id:Option<f64>,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub dd_networkdistribution_id:Option<f64>,
	pub deliverytime_promised:Option<f64>,
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub iscreateplan:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isissue:String,
	pub ismps:Option<String>,
	/// not nullable 
	pub isphantom:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isrequireddrp:String,
	/// not nullable 
	pub isrequiredmrp:String,
	/// not nullable 
	pub m_product_id:f64,
	/// defaults to: (0)::numeric
	pub m_warehouse_id:Option<f64>,
	pub order_max:Option<f64>,
	pub order_min:Option<f64>,
	pub order_pack:Option<f64>,
	pub order_period:Option<f64>,
	pub order_policy:Option<String>,
	pub order_qty:Option<f64>,
	pub planner_id:Option<f64>,
	pub pp_product_bom_id:Option<f64>,
	/// defaults to: (0)::numeric
	pub s_resource_id:Option<f64>,
	pub safetystock:Option<f64>,
	pub timefence:Option<f64>,
	pub transferttime:Option<f64>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub workingtime:Option<f64>,
	/// defaults to: (100)::numeric
	pub yield_:Option<f64>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
}

pub struct PpWfNodeAsset {
	/// primary
	/// not nullable 
	pub pp_wf_node_asset_id:f64,
	/// not nullable 
	pub a_asset_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_wf_node_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub seqno:f64,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_wf_node_id_ad_wf_node:Option<AdWfNode>,
	/// has one
	pub a_asset_id_a_asset:Option<AAsset>,
}

pub struct PpWfNodeProduct {
	/// primary
	/// not nullable 
	pub pp_wf_node_product_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_wf_node_id:f64,
	/// defaults to: 'S'::bpchar
	pub configurationlevel:Option<String>,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'U'::character varying
	/// not nullable 
	pub entitytype:String,
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	pub issubcontracting:Option<String>,
	/// not nullable 
	pub m_product_id:f64,
	pub qty:Option<f64>,
	pub seqno:Option<f64>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub yield_:Option<f64>,
	/// has one
	pub entitytype_ad_entitytype:Option<AdEntitytype>,
	/// has one
	pub ad_wf_node_id_ad_wf_node:Option<AdWfNode>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
}

pub struct QmSpecification {
	/// primary
	/// not nullable 
	pub qm_specification_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_workflow_id:Option<f64>,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub m_attributeset_id:f64,
	/// not nullable 
	pub m_product_id:f64,
	pub name:Option<String>,
	pub pp_product_bom_id:Option<f64>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub validfrom:Option<NaiveDateTime>,
	pub validto:Option<NaiveDateTime>,
	pub value:Option<String>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub pp_product_bom_id_pp_product_bom:Option<PpProductBom>,
	/// has one
	pub ad_workflow_id_ad_workflow:Option<AdWorkflow>,
	/// has one
	pub m_attributeset_id_m_attributeset:Option<MAttributeset>,
	/// has many
	pub qm_specificationline:Option<Vec<QmSpecificationline>>,
}

pub struct QmSpecificationline {
	/// primary
	/// not nullable 
	pub qm_specificationline_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub andor:String,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub m_attribute_id:f64,
	/// not nullable 
	pub operation:String,
	pub qm_specification_id:Option<f64>,
	pub seqno:Option<f64>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub validfrom:Option<String>,
	pub validto:Option<NaiveDateTime>,
	pub value:Option<String>,
	/// has one
	pub m_attribute_id_m_attribute:Option<MAttribute>,
	/// has one
	pub qm_specification_id_qm_specification:Option<QmSpecification>,
}

pub struct RCategory {
	/// primary
	/// not nullable 
	pub r_category_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub m_product_id:Option<f64>,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has many
	pub r_categoryupdates:Option<Vec<RCategoryupdates>>,
	/// has many
	pub r_request:Option<Vec<RRequest>>,
	/// has many
	pub r_requestaction:Option<Vec<RRequestaction>>,
}

pub struct RCategoryupdates {
	/// primary
	/// not nullable 
	pub ad_user_id:f64,
	/// primary
	/// not nullable 
	pub r_category_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isselfservice:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
	/// has one
	pub r_category_id_r_category:Option<RCategory>,
}

pub struct RContactinterest {
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_user_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub optoutdate:Option<NaiveDateTime>,
	/// not nullable 
	pub r_interestarea_id:f64,
	pub subscribedate:Option<NaiveDateTime>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub r_interestarea_id_r_interestarea:Option<RInterestarea>,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
}

pub struct RGroup {
	/// primary
	/// not nullable 
	pub r_group_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub m_bom_id:Option<f64>,
	pub m_changenotice_id:Option<f64>,
	/// not nullable 
	pub name:String,
	pub pp_product_bom_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub m_bom_id_m_bom:Option<MBom>,
	/// has one
	pub m_changenotice_id_m_changenotice:Option<MChangenotice>,
	/// has one
	pub pp_product_bom_id_pp_product_bom:Option<PpProductBom>,
	/// has many
	pub r_groupupdates:Option<Vec<RGroupupdates>>,
	/// has many
	pub r_request:Option<Vec<RRequest>>,
	/// has many
	pub r_requestaction:Option<Vec<RRequestaction>>,
}

pub struct RGroupupdates {
	/// primary
	/// not nullable 
	pub ad_user_id:f64,
	/// primary
	/// not nullable 
	pub r_group_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isselfservice:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
	/// has one
	pub r_group_id_r_group:Option<RGroup>,
}

pub struct RInterestarea {
	/// primary
	/// not nullable 
	pub r_interestarea_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isselfservice:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
	/// has many
	pub ad_ldapaccess:Option<Vec<AdLdapaccess>>,
	/// has many
	pub i_bpartner:Option<Vec<IBpartner>>,
	/// has many
	pub r_contactinterest:Option<Vec<RContactinterest>>,
}

pub struct RIssueknown {
	/// primary
	/// not nullable 
	pub r_issueknown_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub issuestatus:Option<String>,
	/// not nullable 
	pub issuesummary:String,
	/// defaults to: 0
	pub lineno:Option<f64>,
	pub loggername:Option<String>,
	pub processing:Option<String>,
	pub r_issuerecommendation_id:Option<f64>,
	pub r_issuestatus_id:Option<f64>,
	pub r_request_id:Option<f64>,
	/// not nullable 
	pub releaseno:String,
	pub sourceclassname:Option<String>,
	pub sourcemethodname:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub r_issuestatus_id_r_issuestatus:Option<RIssuestatus>,
	/// has one
	pub r_request_id_r_request:Option<RRequest>,
	/// has one
	pub r_issuerecommendation_id_r_issuerecommendation:Option<RIssuerecommendation>,
	/// has many
	pub ad_issue:Option<Vec<AdIssue>>,
}

pub struct RIssueproject {
	/// primary
	/// not nullable 
	pub r_issueproject_id:f64,
	pub a_asset_id:Option<f64>,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub c_project_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	pub profileinfo:Option<String>,
	pub statisticsinfo:Option<String>,
	/// not nullable 
	pub systemstatus:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub a_asset_id_a_asset:Option<AAsset>,
	/// has one
	pub c_project_id_c_project:Option<CProject>,
	/// has many
	pub ad_issue:Option<Vec<AdIssue>>,
	/// has many
	pub r_issuesource:Option<Vec<RIssuesource>>,
}

pub struct RIssuerecommendation {
	/// primary
	/// not nullable 
	pub r_issuerecommendation_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has many
	pub r_issueknown:Option<Vec<RIssueknown>>,
}

pub struct RIssuesource {
	/// primary
	/// not nullable 
	pub r_issuesource_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub profileinfo:Option<String>,
	/// not nullable 
	pub r_issueproject_id:f64,
	/// not nullable 
	pub r_issuesystem_id:f64,
	/// not nullable 
	pub r_issueuser_id:f64,
	pub statisticsinfo:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub r_issuesystem_id_r_issuesystem:Option<RIssuesystem>,
	/// has one
	pub r_issueproject_id_r_issueproject:Option<RIssueproject>,
	/// has one
	pub r_issueuser_id_r_issueuser:Option<RIssueuser>,
}

pub struct RIssuestatus {
	/// primary
	/// not nullable 
	pub r_issuestatus_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has many
	pub r_issueknown:Option<Vec<RIssueknown>>,
}

pub struct RIssuesystem {
	/// primary
	/// not nullable 
	pub r_issuesystem_id:f64,
	pub a_asset_id:Option<f64>,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub dbaddress:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub profileinfo:Option<String>,
	pub statisticsinfo:Option<String>,
	/// not nullable 
	pub systemstatus:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub a_asset_id_a_asset:Option<AAsset>,
	/// has many
	pub ad_issue:Option<Vec<AdIssue>>,
	/// has many
	pub r_issuesource:Option<Vec<RIssuesource>>,
}

pub struct RIssueuser {
	/// primary
	/// not nullable 
	pub r_issueuser_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_user_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub username:String,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
	/// has many
	pub ad_issue:Option<Vec<AdIssue>>,
	/// has many
	pub r_issuesource:Option<Vec<RIssuesource>>,
}

pub struct RMailtext {
	/// primary
	/// not nullable 
	pub r_mailtext_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub ishtml:String,
	pub mailheader:Option<String>,
	/// not nullable 
	pub mailtext:String,
	pub mailtext2:Option<String>,
	pub mailtext3:Option<String>,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has many
	pub ad_printform:Option<Vec<AdPrintform>>,
	/// has many
	pub ad_usermail:Option<Vec<AdUsermail>>,
	/// has many
	pub ad_wf_node:Option<Vec<AdWfNode>>,
	/// has many
	pub m_product:Option<Vec<MProduct>>,
	/// has many
	pub r_mailtext_trl:Option<Vec<RMailtextTrl>>,
	/// has many
	pub r_request:Option<Vec<RRequest>>,
}

pub struct RMailtextTrl {
	/// primary
	/// not nullable 
	pub r_mailtext_id:f64,
	/// primary
	/// not nullable 
	pub ad_language:String,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub istranslated:String,
	pub mailheader:Option<String>,
	/// not nullable 
	pub mailtext:String,
	pub mailtext2:Option<String>,
	pub mailtext3:Option<String>,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub r_mailtext_id_r_mailtext:Option<RMailtext>,
	/// has one
	pub ad_language_ad_language:Option<AdLanguage>,
}

pub struct RRequest {
	/// primary
	/// not nullable 
	pub r_request_id:f64,
	pub a_asset_id:Option<f64>,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_role_id:Option<f64>,
	pub ad_table_id:Option<f64>,
	pub ad_user_id:Option<f64>,
	pub c_activity_id:Option<f64>,
	pub c_bpartner_id:Option<f64>,
	pub c_campaign_id:Option<f64>,
	pub c_invoice_id:Option<f64>,
	pub c_invoicerequest_id:Option<f64>,
	pub c_order_id:Option<f64>,
	pub c_payment_id:Option<f64>,
	pub c_project_id:Option<f64>,
	pub closedate:Option<NaiveDateTime>,
	/// defaults to: 'C'::bpchar
	/// not nullable 
	pub confidentialtype:String,
	/// not nullable 
	pub confidentialtypeentry:String,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub datecompleteplan:Option<NaiveDateTime>,
	pub datelastaction:Option<NaiveDateTime>,
	pub datelastalert:Option<NaiveDateTime>,
	pub datenextaction:Option<NaiveDateTime>,
	pub datestartplan:Option<NaiveDateTime>,
	/// not nullable 
	pub documentno:String,
	/// not nullable 
	pub duetype:String,
	pub endtime:Option<NaiveDateTime>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isescalated:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isinvoiced:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isselfservice:String,
	pub lastresult:Option<String>,
	pub m_changerequest_id:Option<f64>,
	pub m_fixchangenotice_id:Option<f64>,
	pub m_inout_id:Option<f64>,
	pub m_product_id:Option<f64>,
	pub m_productspent_id:Option<f64>,
	pub m_rma_id:Option<f64>,
	pub nextaction:Option<String>,
	/// not nullable 
	pub priority:String,
	pub priorityuser:Option<String>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	/// defaults to: 0
	pub qtyinvoiced:Option<f64>,
	pub qtyplan:Option<f64>,
	/// defaults to: 0
	pub qtyspent:Option<f64>,
	pub r_category_id:Option<f64>,
	pub r_group_id:Option<f64>,
	pub r_mailtext_id:Option<f64>,
	pub r_requestrelated_id:Option<f64>,
	/// not nullable 
	pub r_requesttype_id:f64,
	pub r_resolution_id:Option<f64>,
	pub r_standardresponse_id:Option<f64>,
	pub r_status_id:Option<f64>,
	pub record_id:Option<f64>,
	/// defaults to: 0
	/// not nullable 
	pub requestamt:f64,
	pub result:Option<String>,
	/// not nullable 
	pub salesrep_id:f64,
	pub startdate:Option<NaiveDateTime>,
	pub starttime:Option<NaiveDateTime>,
	/// not nullable 
	pub summary:String,
	pub taskstatus:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub r_requesttype_id_r_requesttype:Option<RRequesttype>,
	/// has one
	pub r_group_id_r_group:Option<RGroup>,
	/// has one
	pub r_category_id_r_category:Option<RCategory>,
	/// has one
	pub r_status_id_r_status:Option<RStatus>,
	/// has one
	pub r_resolution_id_r_resolution:Option<RResolution>,
	/// has one, self referential
	pub r_requestrelated_id_r_request:Option<Box<RRequest>>,
	/// has one
	pub salesrep_id_ad_user:Option<AdUser>,
	/// has one
	pub ad_role_id_ad_role:Option<AdRole>,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
	/// has one
	pub c_campaign_id_c_campaign:Option<CCampaign>,
	/// has one
	pub c_order_id_c_order:Option<COrder>,
	/// has one
	pub c_invoice_id_c_invoice:Option<CInvoice>,
	/// has one
	pub c_payment_id_c_payment:Option<CPayment>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub c_project_id_c_project:Option<CProject>,
	/// has one
	pub a_asset_id_a_asset:Option<AAsset>,
	/// has one
	pub m_inout_id_m_inout:Option<MInout>,
	/// has one
	pub m_rma_id_m_rma:Option<MRma>,
	/// has one
	pub ad_table_id_ad_table:Option<AdTable>,
	/// has one
	pub r_mailtext_id_r_mailtext:Option<RMailtext>,
	/// has one
	pub r_standardresponse_id_r_standardresponse:Option<RStandardresponse>,
	/// has one
	pub m_productspent_id_m_product:Option<MProduct>,
	/// has one
	pub c_activity_id_c_activity:Option<CActivity>,
	/// has one
	pub c_invoicerequest_id_c_invoice:Option<CInvoice>,
	/// has one
	pub m_changerequest_id_m_changerequest:Option<MChangerequest>,
	/// has one
	pub m_fixchangenotice_id_m_changenotice:Option<MChangenotice>,
	/// has many
	pub ad_issue:Option<Vec<AdIssue>>,
	/// has many
	pub r_issueknown:Option<Vec<RIssueknown>>,
	/// has many
	pub r_request:Option<Vec<RRequest>>,
	/// has many
	pub r_requestaction:Option<Vec<RRequestaction>>,
	/// has many
	pub r_requestupdate:Option<Vec<RRequestupdate>>,
	/// has many
	pub r_requestupdates:Option<Vec<RRequestupdates>>,
}

pub struct RRequestaction {
	/// primary
	/// not nullable 
	pub r_requestaction_id:f64,
	pub a_asset_id:Option<f64>,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_role_id:Option<f64>,
	pub ad_user_id:Option<f64>,
	pub c_activity_id:Option<f64>,
	pub c_bpartner_id:Option<f64>,
	pub c_invoice_id:Option<f64>,
	pub c_order_id:Option<f64>,
	pub c_payment_id:Option<f64>,
	pub c_project_id:Option<f64>,
	pub confidentialtype:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub datecompleteplan:Option<NaiveDateTime>,
	pub datenextaction:Option<NaiveDateTime>,
	pub datestartplan:Option<NaiveDateTime>,
	pub enddate:Option<NaiveDateTime>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub isescalated:Option<String>,
	pub isinvoiced:Option<String>,
	pub isselfservice:Option<String>,
	pub m_inout_id:Option<f64>,
	pub m_product_id:Option<f64>,
	pub m_productspent_id:Option<f64>,
	pub m_rma_id:Option<f64>,
	pub nullcolumns:Option<String>,
	pub priority:Option<String>,
	pub priorityuser:Option<String>,
	pub qtyinvoiced:Option<f64>,
	pub qtyplan:Option<f64>,
	pub qtyspent:Option<f64>,
	pub r_category_id:Option<f64>,
	pub r_group_id:Option<f64>,
	/// not nullable 
	pub r_request_id:f64,
	pub r_requesttype_id:Option<f64>,
	pub r_resolution_id:Option<f64>,
	pub r_status_id:Option<f64>,
	pub salesrep_id:Option<f64>,
	pub startdate:Option<NaiveDateTime>,
	pub summary:Option<String>,
	pub taskstatus:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub r_request_id_r_request:Option<RRequest>,
	/// has one
	pub r_group_id_r_group:Option<RGroup>,
	/// has one
	pub r_category_id_r_category:Option<RCategory>,
	/// has one
	pub r_status_id_r_status:Option<RStatus>,
	/// has one
	pub r_resolution_id_r_resolution:Option<RResolution>,
	/// has one
	pub salesrep_id_ad_user:Option<AdUser>,
	/// has one
	pub ad_role_id_ad_role:Option<AdRole>,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
	/// has one
	pub c_order_id_c_order:Option<COrder>,
	/// has one
	pub c_invoice_id_c_invoice:Option<CInvoice>,
	/// has one
	pub c_payment_id_c_payment:Option<CPayment>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub c_project_id_c_project:Option<CProject>,
	/// has one
	pub c_activity_id_c_activity:Option<CActivity>,
	/// has one
	pub r_requesttype_id_r_requesttype:Option<RRequesttype>,
	/// has one
	pub a_asset_id_a_asset:Option<AAsset>,
	/// has one
	pub m_inout_id_m_inout:Option<MInout>,
	/// has one
	pub m_rma_id_m_rma:Option<MRma>,
	/// has one
	pub m_productspent_id_m_product:Option<MProduct>,
}

pub struct RRequestprocessor {
	/// primary
	/// not nullable 
	pub r_requestprocessor_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub datelastrun:Option<NaiveDateTime>,
	pub datenextrun:Option<NaiveDateTime>,
	pub description:Option<String>,
	/// not nullable 
	pub frequency:f64,
	/// not nullable 
	pub frequencytype:String,
	/// defaults to: 0
	/// not nullable 
	pub inactivityalertdays:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub keeplogdays:f64,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub overduealertdays:f64,
	/// not nullable 
	pub overdueassigndays:f64,
	pub processing:Option<String>,
	pub r_requesttype_id:Option<f64>,
	/// defaults to: 7
	/// not nullable 
	pub reminddays:f64,
	/// not nullable 
	pub supervisor_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub supervisor_id_ad_user:Option<AdUser>,
	/// has one
	pub r_requesttype_id_r_requesttype:Option<RRequesttype>,
	/// has many
	pub r_requestprocessor_route:Option<Vec<RRequestprocessorRoute>>,
	/// has many
	pub r_requestprocessorlog:Option<Vec<RRequestprocessorlog>>,
}

pub struct RRequestprocessorRoute {
	/// primary
	/// not nullable 
	pub r_requestprocessor_route_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_user_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub keyword:Option<String>,
	/// not nullable 
	pub r_requestprocessor_id:f64,
	pub r_requesttype_id:Option<f64>,
	/// not nullable 
	pub seqno:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub r_requestprocessor_id_r_requestprocessor:Option<RRequestprocessor>,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
	/// has one
	pub r_requesttype_id_r_requesttype:Option<RRequesttype>,
}

pub struct RRequestprocessorlog {
	/// primary
	/// not nullable 
	pub r_requestprocessor_id:f64,
	/// primary
	/// not nullable 
	pub r_requestprocessorlog_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub binarydata:Option<Vec<u8>>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub iserror:String,
	pub reference:Option<String>,
	pub summary:Option<String>,
	pub textmsg:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub r_requestprocessor_id_r_requestprocessor:Option<RRequestprocessor>,
}

pub struct RRequesttype {
	/// primary
	/// not nullable 
	pub r_requesttype_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub autoduedatedays:Option<f64>,
	/// defaults to: 'C'::bpchar
	/// not nullable 
	pub confidentialtype:String,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 7
	/// not nullable 
	pub duedatetolerance:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isautochangerequest:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isconfidentialinfo:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isdefault:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isemailwhendue:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isemailwhenoverdue:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isindexed:String,
	pub isinvoiced:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isselfservice:String,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub r_statuscategory_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub r_statuscategory_id_r_statuscategory:Option<RStatuscategory>,
	/// has many
	pub ad_userbpaccess:Option<Vec<AdUserbpaccess>>,
	/// has many
	pub k_index:Option<Vec<KIndex>>,
	/// has many
	pub k_indexstop:Option<Vec<KIndexstop>>,
	/// has many
	pub pa_measure:Option<Vec<PaMeasure>>,
	/// has many
	pub r_request:Option<Vec<RRequest>>,
	/// has many
	pub r_requestaction:Option<Vec<RRequestaction>>,
	/// has many
	pub r_requestprocessor:Option<Vec<RRequestprocessor>>,
	/// has many
	pub r_requestprocessor_route:Option<Vec<RRequestprocessorRoute>>,
	/// has many
	pub r_requesttypeupdates:Option<Vec<RRequesttypeupdates>>,
}

pub struct RRequesttypeupdates {
	/// primary
	/// not nullable 
	pub ad_user_id:f64,
	/// primary
	/// not nullable 
	pub r_requesttype_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isselfservice:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
	/// has one
	pub r_requesttype_id_r_requesttype:Option<RRequesttype>,
}

pub struct RRequestupdate {
	/// primary
	/// not nullable 
	pub r_requestupdate_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub confidentialtypeentry:String,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub endtime:Option<NaiveDateTime>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub m_productspent_id:Option<f64>,
	/// defaults to: 0
	pub qtyinvoiced:Option<f64>,
	/// defaults to: 0
	pub qtyspent:Option<f64>,
	/// not nullable 
	pub r_request_id:f64,
	pub result:Option<String>,
	pub starttime:Option<NaiveDateTime>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub r_request_id_r_request:Option<RRequest>,
	/// has one
	pub m_productspent_id_m_product:Option<MProduct>,
}

pub struct RRequestupdates {
	/// primary
	/// not nullable 
	pub ad_user_id:f64,
	/// primary
	/// not nullable 
	pub r_request_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isselfservice:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
	/// has one
	pub r_request_id_r_request:Option<RRequest>,
}

pub struct RResolution {
	/// primary
	/// not nullable 
	pub r_resolution_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has many
	pub r_request:Option<Vec<RRequest>>,
	/// has many
	pub r_requestaction:Option<Vec<RRequestaction>>,
}

pub struct RStandardresponse {
	/// primary
	/// not nullable 
	pub r_standardresponse_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub responsetext:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has many
	pub r_request:Option<Vec<RRequest>>,
}

pub struct RStatus {
	/// primary
	/// not nullable 
	pub r_status_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isclosed:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isdefault:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isfinalclose:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isopen:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub iswebcanupdate:String,
	/// not nullable 
	pub name:String,
	pub next_status_id:Option<f64>,
	/// not nullable 
	pub r_statuscategory_id:f64,
	/// defaults to: 0
	/// not nullable 
	pub seqno:f64,
	pub timeoutdays:Option<f64>,
	pub update_status_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
	/// has one, self referential
	pub next_status_id_r_status:Option<Box<RStatus>>,
	/// has one, self referential
	pub update_status_id_r_status:Option<Box<RStatus>>,
	/// has one
	pub r_statuscategory_id_r_statuscategory:Option<RStatuscategory>,
	/// has many
	pub r_request:Option<Vec<RRequest>>,
	/// has many
	pub r_requestaction:Option<Vec<RRequestaction>>,
	/// has many
	pub r_status:Option<Vec<RStatus>>,
}

pub struct RStatuscategory {
	/// primary
	/// not nullable 
	pub r_statuscategory_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isdefault:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has many
	pub r_requesttype:Option<Vec<RRequesttype>>,
	/// has many
	pub r_status:Option<Vec<RStatus>>,
}

pub struct Relationship {
	/// primary
	/// not nullable 
	pub relationship_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: NULL::character varying
	pub description:Option<String>,
	/// defaults to: NULL::character varying
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
}

pub struct SExpensetype {
	/// primary
	/// not nullable 
	pub s_expensetype_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_taxcategory_id:f64,
	/// not nullable 
	pub c_uom_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isinvoiced:String,
	/// not nullable 
	pub m_product_category_id:f64,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
	/// has one
	pub c_uom_id_c_uom:Option<CUom>,
	/// has one
	pub m_product_category_id_m_product_category:Option<MProductCategory>,
	/// has one
	pub c_taxcategory_id_c_taxcategory:Option<CTaxcategory>,
	/// has many
	pub m_product:Option<Vec<MProduct>>,
}

pub struct SResource {
	/// primary
	/// not nullable 
	pub s_resource_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_user_id:Option<f64>,
	/// defaults to: 0
	pub chargeableqty:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub dailycapacity:Option<f64>,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isavailable:String,
	/// defaults to: 'N'::bpchar
	pub ismanufacturingresource:Option<String>,
	/// not nullable 
	pub m_warehouse_id:f64,
	pub manufacturingresourcetype:Option<String>,
	/// not nullable 
	pub name:String,
	/// defaults to: (100)::numeric
	/// not nullable 
	pub percentutilization:f64,
	/// defaults to: (0)::numeric
	pub planninghorizon:Option<f64>,
	pub queuingtime:Option<f64>,
	/// not nullable 
	pub s_resourcetype_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
	pub waitingtime:Option<f64>,
	/// has one
	pub s_resourcetype_id_s_resourcetype:Option<SResourcetype>,
	/// has one
	pub m_warehouse_id_m_warehouse:Option<MWarehouse>,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
	/// has many
	pub ad_wf_node:Option<Vec<AdWfNode>>,
	/// has many
	pub ad_workflow:Option<Vec<AdWorkflow>>,
	/// has many
	pub m_product:Option<Vec<MProduct>>,
	/// has many
	pub pp_cost_collector:Option<Vec<PpCostCollector>>,
	/// has many
	pub pp_mrp:Option<Vec<PpMrp>>,
	/// has many
	pub pp_order:Option<Vec<PpOrder>>,
	/// has many
	pub pp_order_node:Option<Vec<PpOrderNode>>,
	/// has many
	pub pp_order_workflow:Option<Vec<PpOrderWorkflow>>,
	/// has many
	pub s_resourceassignment:Option<Vec<SResourceassignment>>,
	/// has many
	pub s_resourceunavailable:Option<Vec<SResourceunavailable>>,
}

pub struct SResourceassignment {
	/// primary
	/// not nullable 
	pub s_resourceassignment_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub assigndatefrom:NaiveDateTime,
	pub assigndateto:Option<NaiveDateTime>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isconfirmed:String,
	/// not nullable 
	pub name:String,
	/// defaults to: 0
	pub qty:Option<f64>,
	/// not nullable 
	pub s_resource_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub s_resource_id_s_resource:Option<SResource>,
	/// has many
	pub c_invoiceline:Option<Vec<CInvoiceline>>,
	/// has many
	pub c_orderline:Option<Vec<COrderline>>,
	/// has many
	pub s_timeexpenseline:Option<Vec<STimeexpenseline>>,
}

pub struct SResourcetype {
	/// primary
	/// not nullable 
	pub s_resourcetype_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub allowuomfractions:String,
	/// not nullable 
	pub c_taxcategory_id:f64,
	/// not nullable 
	pub c_uom_id:f64,
	/// defaults to: 0
	pub chargeableqty:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isdateslot:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub issingleassignment:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub istimeslot:String,
	/// not nullable 
	pub m_product_category_id:f64,
	/// not nullable 
	pub name:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub onfriday:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub onmonday:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub onsaturday:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub onsunday:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub onthursday:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub ontuesday:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub onwednesday:String,
	pub timeslotend:Option<NaiveDateTime>,
	pub timeslotstart:Option<NaiveDateTime>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
	/// has one
	pub c_uom_id_c_uom:Option<CUom>,
	/// has one
	pub m_product_category_id_m_product_category:Option<MProductCategory>,
	/// has one
	pub c_taxcategory_id_c_taxcategory:Option<CTaxcategory>,
	/// has many
	pub s_resource:Option<Vec<SResource>>,
}

pub struct SResourceunavailable {
	/// primary
	/// not nullable 
	pub s_resourceunavailable_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub datefrom:NaiveDateTime,
	pub dateto:Option<NaiveDateTime>,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub s_resource_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub s_resource_id_s_resource:Option<SResource>,
}

pub struct STimeexpense {
	/// primary
	/// not nullable 
	pub s_timeexpense_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub approvalamt:Option<f64>,
	/// not nullable 
	pub c_bpartner_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub datereport:NaiveDateTime,
	pub description:Option<String>,
	/// not nullable 
	pub docaction:String,
	/// not nullable 
	pub docstatus:String,
	/// not nullable 
	pub documentno:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isapproved:String,
	/// not nullable 
	pub m_pricelist_id:f64,
	/// not nullable 
	pub m_warehouse_id:f64,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	pub processing:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub m_pricelist_id_m_pricelist:Option<MPricelist>,
	/// has one
	pub m_warehouse_id_m_warehouse:Option<MWarehouse>,
	/// has many
	pub s_timeexpenseline:Option<Vec<STimeexpenseline>>,
}

pub struct STimeexpenseline {
	/// primary
	/// not nullable 
	pub s_timeexpenseline_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub c_activity_id:Option<f64>,
	pub c_bpartner_id:Option<f64>,
	pub c_campaign_id:Option<f64>,
	pub c_currency_id:Option<f64>,
	pub c_invoiceline_id:Option<f64>,
	pub c_orderline_id:Option<f64>,
	pub c_project_id:Option<f64>,
	pub c_projectphase_id:Option<f64>,
	pub c_projecttask_id:Option<f64>,
	pub c_uom_id:Option<f64>,
	/// defaults to: 0
	pub convertedamt:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub dateexpense:Option<NaiveDateTime>,
	pub description:Option<String>,
	/// defaults to: 0
	pub expenseamt:Option<f64>,
	/// defaults to: 0
	pub invoiceprice:Option<f64>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isinvoiced:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub istimereport:String,
	/// not nullable 
	pub line:f64,
	pub m_product_id:Option<f64>,
	pub note:Option<String>,
	pub priceinvoiced:Option<f64>,
	pub pricereimbursed:Option<f64>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	/// defaults to: 0
	pub qty:Option<f64>,
	pub qtyinvoiced:Option<f64>,
	pub qtyreimbursed:Option<f64>,
	pub s_resourceassignment_id:Option<f64>,
	/// not nullable 
	pub s_timeexpense_id:f64,
	pub s_timetype_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub s_timeexpense_id_s_timeexpense:Option<STimeexpense>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub c_currency_id_c_currency:Option<CCurrency>,
	/// has one
	pub s_resourceassignment_id_s_resourceassignment:Option<SResourceassignment>,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub c_project_id_c_project:Option<CProject>,
	/// has one
	pub c_activity_id_c_activity:Option<CActivity>,
	/// has one
	pub c_campaign_id_c_campaign:Option<CCampaign>,
	/// has one
	pub c_invoiceline_id_c_invoiceline:Option<CInvoiceline>,
	/// has one
	pub c_uom_id_c_uom:Option<CUom>,
	/// has one
	pub c_orderline_id_c_orderline:Option<COrderline>,
	/// has one
	pub c_projectphase_id_c_projectphase:Option<CProjectphase>,
	/// has one
	pub c_projecttask_id_c_projecttask:Option<CProjecttask>,
	/// has one
	pub s_timetype_id_s_timetype:Option<STimetype>,
	/// has many
	pub c_projectissue:Option<Vec<CProjectissue>>,
}

pub struct STimetype {
	/// primary
	/// not nullable 
	pub s_timetype_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has many
	pub s_timeexpenseline:Option<Vec<STimeexpenseline>>,
}

pub struct STraining {
	/// primary
	/// not nullable 
	pub s_training_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_taxcategory_id:f64,
	/// not nullable 
	pub c_uom_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub descriptionurl:Option<String>,
	pub documentnote:Option<String>,
	pub help:Option<String>,
	pub imageurl:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub m_product_category_id:f64,
	/// not nullable 
	pub name:String,
	pub processing:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub m_product_category_id_m_product_category:Option<MProductCategory>,
	/// has one
	pub c_taxcategory_id_c_taxcategory:Option<CTaxcategory>,
	/// has one
	pub c_uom_id_c_uom:Option<CUom>,
	/// has many
	pub s_training_class:Option<Vec<STrainingClass>>,
}

pub struct STrainingClass {
	/// primary
	/// not nullable 
	pub s_training_class_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub enddate:NaiveDateTime,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub m_product_id:f64,
	/// not nullable 
	pub s_training_id:f64,
	/// not nullable 
	pub startdate:NaiveDateTime,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub s_training_id_s_training:Option<STraining>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
}

pub struct TAging {
	/// primary
	/// not nullable 
	pub ad_pinstance_id:f64,
	/// primary
	/// not nullable 
	pub c_bpartner_id:f64,
	/// primary
	/// not nullable 
	pub c_currency_id:f64,
	/// primary
	/// not nullable 
	pub c_invoice_id:f64,
	/// primary
	/// not nullable 
	pub c_invoicepayschedule_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub c_activity_id:Option<f64>,
	/// not nullable 
	pub c_bp_group_id:f64,
	pub c_campaign_id:Option<f64>,
	pub c_project_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'N'::bpchar
	pub dateacct:Option<String>,
	/// defaults to: 0
	pub daysdue:Option<f64>,
	/// defaults to: 0
	/// not nullable 
	pub due0:f64,
	/// defaults to: 0
	/// not nullable 
	pub due0_30:f64,
	/// defaults to: 0
	/// not nullable 
	pub due0_7:f64,
	/// defaults to: 0
	/// not nullable 
	pub due1_7:f64,
	/// defaults to: 0
	/// not nullable 
	pub due31_60:f64,
	/// defaults to: 0
	/// not nullable 
	pub due31_plus:f64,
	/// defaults to: 0
	/// not nullable 
	pub due61_90:f64,
	/// defaults to: 0
	/// not nullable 
	pub due61_plus:f64,
	/// defaults to: 0
	/// not nullable 
	pub due8_30:f64,
	/// defaults to: 0
	/// not nullable 
	pub due91_plus:f64,
	/// defaults to: 0
	/// not nullable 
	pub dueamt:f64,
	/// not nullable 
	pub duedate:NaiveDateTime,
	/// defaults to: 0
	/// not nullable 
	pub invoicedamt:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub islistinvoices:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub issotrx:String,
	/// defaults to: 0
	/// not nullable 
	pub openamt:f64,
	/// defaults to: 0
	/// not nullable 
	pub pastdue1_30:f64,
	/// defaults to: 0
	/// not nullable 
	pub pastdue1_7:f64,
	/// defaults to: 0
	/// not nullable 
	pub pastdue31_60:f64,
	/// defaults to: 0
	/// not nullable 
	pub pastdue31_plus:f64,
	/// defaults to: 0
	/// not nullable 
	pub pastdue61_90:f64,
	/// defaults to: 0
	/// not nullable 
	pub pastdue61_plus:f64,
	/// defaults to: 0
	/// not nullable 
	pub pastdue8_30:f64,
	/// defaults to: 0
	/// not nullable 
	pub pastdue91_plus:f64,
	/// defaults to: 0
	/// not nullable 
	pub pastdueamt:f64,
	/// not nullable 
	pub statementdate:NaiveDateTime,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_pinstance_id_ad_pinstance:Option<AdPinstance>,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub c_currency_id_c_currency:Option<CCurrency>,
	/// has one
	pub c_bp_group_id_c_bp_group:Option<CBpGroup>,
	/// has one
	pub c_project_id_c_project:Option<CProject>,
	/// has one
	pub c_campaign_id_c_campaign:Option<CCampaign>,
	/// has one
	pub c_activity_id_c_activity:Option<CActivity>,
}

pub struct TAlterColumn {
	pub columnname:Option<String>,
	pub datatype:Option<String>,
	pub defaultclause:Option<String>,
	pub nullclause:Option<String>,
	pub tablename:Option<String>,
}

pub struct TBomline {
	/// primary
	/// not nullable 
	pub t_bomline_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_pinstance_id:Option<f64>,
	pub c_acctschema_id:Option<f64>,
	pub cost:Option<String>,
	/// defaults to: NULL::bpchar
	pub costingmethod:Option<String>,
	pub coststandard:Option<f64>,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub currentcostprice:Option<f64>,
	pub currentcostpricell:Option<f64>,
	pub futurecostprice:Option<f64>,
	pub futurecostpricell:Option<f64>,
	/// defaults to: 'N'::bpchar
	pub implosion:Option<String>,
	pub isactive:Option<String>,
	/// defaults to: NULL::bpchar
	pub iscostfrozen:Option<String>,
	pub levelno:Option<f64>,
	pub levels:Option<String>,
	pub m_costelement_id:Option<f64>,
	/// defaults to: NULL::numeric
	pub m_costtype_id:Option<f64>,
	pub m_product_id:Option<f64>,
	pub pp_product_bom_id:Option<f64>,
	pub pp_product_bomline_id:Option<f64>,
	pub qtybom:Option<f64>,
	/// not nullable 
	pub sel_product_id:f64,
	pub seqno:Option<f64>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
}

pub struct TDistributionrundetail {
	/// primary
	/// not nullable 
	pub m_distributionrun_id:f64,
	/// primary
	/// not nullable 
	pub m_distributionrunline_id:f64,
	/// primary
	/// not nullable 
	pub m_distributionlist_id:f64,
	/// primary
	/// not nullable 
	pub m_distributionlistline_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_bpartner_id:f64,
	/// not nullable 
	pub c_bpartner_location_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub m_product_id:f64,
	/// defaults to: 0
	/// not nullable 
	pub minqty:f64,
	/// defaults to: 0
	/// not nullable 
	pub qty:f64,
	/// not nullable 
	pub ratio:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub m_distributionrun_id_m_distributionrun:Option<MDistributionrun>,
	/// has one
	pub m_distributionrunline_id_m_distributionrunline:Option<MDistributionrunline>,
	/// has one
	pub m_distributionlist_id_m_distributionlist:Option<MDistributionlist>,
	/// has one
	pub m_distributionlistline_id_m_distributionlistline:Option<MDistributionlistline>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub c_bpartner_location_id_c_bpartner_location:Option<CBpartnerLocation>,
}

pub struct TInventoryvalue {
	/// primary
	/// not nullable 
	pub ad_pinstance_id:f64,
	/// primary
	/// not nullable 
	pub m_warehouse_id:f64,
	/// primary
	/// not nullable 
	pub m_product_id:f64,
	/// primary
	/// not nullable 
	pub m_attributesetinstance_id:f64,
	pub ad_client_id:Option<f64>,
	pub ad_org_id:Option<f64>,
	pub c_currency_id:Option<f64>,
	/// defaults to: 0
	pub cost:Option<f64>,
	/// defaults to: 0
	pub costamt:Option<f64>,
	/// defaults to: 0
	pub coststandard:Option<f64>,
	/// defaults to: 0
	pub coststandardamt:Option<f64>,
	pub datevalue:Option<NaiveDateTime>,
	pub m_costelement_id:Option<f64>,
	pub m_pricelist_version_id:Option<f64>,
	/// defaults to: 0
	pub pricelimit:Option<f64>,
	/// defaults to: 0
	pub pricelimitamt:Option<f64>,
	/// defaults to: 0
	pub pricelist:Option<f64>,
	/// defaults to: 0
	pub pricelistamt:Option<f64>,
	/// defaults to: 0
	pub pricepo:Option<f64>,
	/// defaults to: 0
	pub pricepoamt:Option<f64>,
	/// defaults to: 0
	pub pricestd:Option<f64>,
	/// defaults to: 0
	pub pricestdamt:Option<f64>,
	/// defaults to: 0
	pub qtyonhand:Option<f64>,
	/// has one
	pub ad_pinstance_id_ad_pinstance:Option<AdPinstance>,
	/// has one
	pub m_warehouse_id_m_warehouse:Option<MWarehouse>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub m_attributesetinstance_id_m_attributesetinstance:Option<MAttributesetinstance>,
	/// has one
	pub m_pricelist_version_id_m_pricelist_version:Option<MPricelistVersion>,
	/// has one
	pub c_currency_id_c_currency:Option<CCurrency>,
	/// has one
	pub m_costelement_id_m_costelement:Option<MCostelement>,
}

pub struct TInvoicegl {
	/// primary
	/// not nullable 
	pub ad_pinstance_id:f64,
	/// primary
	/// not nullable 
	pub c_invoice_id:f64,
	/// primary
	/// not nullable 
	pub fact_acct_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: 0
	/// not nullable 
	pub amtacctbalance:f64,
	/// defaults to: 0
	/// not nullable 
	pub amtrevalcr:f64,
	/// defaults to: 0
	/// not nullable 
	pub amtrevalcrdiff:f64,
	/// defaults to: 0
	/// not nullable 
	pub amtrevaldr:f64,
	/// defaults to: 0
	/// not nullable 
	pub amtrevaldrdiff:f64,
	/// defaults to: 0
	/// not nullable 
	pub amtsourcebalance:f64,
	pub apar:Option<String>,
	/// not nullable 
	pub c_conversiontypereval_id:f64,
	pub c_doctypereval_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub datereval:NaiveDateTime,
	/// defaults to: 0
	/// not nullable 
	pub grandtotal:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isallcurrencies:String,
	/// defaults to: 0
	/// not nullable 
	pub openamt:f64,
	pub percent:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_pinstance_id_ad_pinstance:Option<AdPinstance>,
	/// has one
	pub c_invoice_id_c_invoice:Option<CInvoice>,
	/// has one
	pub fact_acct_id_fact_acct:Option<FactAcct>,
	/// has one
	pub c_conversiontypereval_id_c_conversiontype:Option<CConversiontype>,
	/// has one
	pub c_doctypereval_id_c_doctype:Option<CDoctype>,
}

pub struct TMrpCrp {
	/// primary
	/// not nullable 
	pub t_mrp_crp_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_pinstance_id:Option<f64>,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub isactive:Option<String>,
	pub seqno:Option<f64>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
}

pub struct TReplenish {
	/// primary
	/// not nullable 
	pub ad_pinstance_id:f64,
	/// primary
	/// not nullable 
	pub m_warehouse_id:f64,
	/// primary
	/// not nullable 
	pub m_product_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub c_bpartner_id:f64,
	pub c_doctype_id:Option<f64>,
	/// defaults to: 0
	/// not nullable 
	pub level_max:f64,
	/// defaults to: 0
	/// not nullable 
	pub level_min:f64,
	pub m_warehousesource_id:Option<f64>,
	/// defaults to: 0
	/// not nullable 
	pub order_min:f64,
	/// defaults to: 0
	/// not nullable 
	pub order_pack:f64,
	/// defaults to: 0
	/// not nullable 
	pub qtyonhand:f64,
	/// defaults to: 0
	/// not nullable 
	pub qtyordered:f64,
	/// defaults to: 0
	/// not nullable 
	pub qtyreserved:f64,
	/// defaults to: 0
	/// not nullable 
	pub qtytoorder:f64,
	pub replenishmentcreate:Option<String>,
	/// not nullable 
	pub replenishtype:String,
	pub updated:Option<NaiveDateTime>,
	pub updatedby:Option<f64>,
	/// has one
	pub ad_pinstance_id_ad_pinstance:Option<AdPinstance>,
	/// has one
	pub m_warehouse_id_m_warehouse:Option<MWarehouse>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub m_warehousesource_id_m_warehouse:Option<MWarehouse>,
	/// has one
	pub c_doctype_id_c_doctype:Option<CDoctype>,
}

pub struct TReport {
	/// primary
	/// not nullable 
	pub ad_pinstance_id:f64,
	/// primary
	/// not nullable 
	pub pa_reportline_id:f64,
	/// primary
	/// not nullable 
	pub record_id:f64,
	/// primary
	/// not nullable 
	pub fact_acct_id:f64,
	pub col_0:Option<f64>,
	pub col_1:Option<f64>,
	pub col_10:Option<f64>,
	pub col_11:Option<f64>,
	pub col_12:Option<f64>,
	pub col_13:Option<f64>,
	pub col_14:Option<f64>,
	pub col_15:Option<f64>,
	pub col_16:Option<f64>,
	pub col_17:Option<f64>,
	pub col_18:Option<f64>,
	pub col_19:Option<f64>,
	pub col_2:Option<f64>,
	pub col_20:Option<f64>,
	pub col_3:Option<f64>,
	pub col_4:Option<f64>,
	pub col_5:Option<f64>,
	pub col_6:Option<f64>,
	pub col_7:Option<f64>,
	pub col_8:Option<f64>,
	pub col_9:Option<f64>,
	pub description:Option<String>,
	/// defaults to: 0
	pub levelno:Option<f64>,
	pub name:Option<String>,
	pub seqno:Option<f64>,
	/// has one
	pub ad_pinstance_id_ad_pinstance:Option<AdPinstance>,
	/// has one
	pub pa_reportline_id_pa_reportline:Option<PaReportline>,
}

pub struct TReportstatement {
	/// primary
	/// not nullable 
	pub ad_pinstance_id:f64,
	/// primary
	/// not nullable 
	pub fact_acct_id:f64,
	/// defaults to: 0
	pub amtacctcr:Option<f64>,
	/// defaults to: 0
	pub amtacctdr:Option<f64>,
	/// defaults to: 0
	pub balance:Option<f64>,
	/// not nullable 
	pub dateacct:NaiveDateTime,
	pub description:Option<String>,
	/// not nullable 
	pub levelno:f64,
	pub name:Option<String>,
	/// defaults to: 0
	pub qty:Option<f64>,
	/// has one
	pub ad_pinstance_id_ad_pinstance:Option<AdPinstance>,
}

pub struct TSelection {
	/// primary
	/// not nullable 
	pub ad_pinstance_id:f64,
	/// primary
	/// not nullable 
	pub t_selection_id:f64,
}

pub struct TSelection2 {
	/// primary
	/// not nullable 
	pub ad_pinstance_id:f64,
	/// primary
	/// not nullable 
	pub query_id:f64,
	/// primary
	/// not nullable 
	pub t_selection_id:f64,
}

pub struct TSpool {
	/// primary
	/// not nullable 
	pub ad_pinstance_id:f64,
	/// primary
	/// not nullable 
	pub seqno:f64,
	/// not nullable 
	pub msgtext:String,
	/// has one
	pub ad_pinstance_id_ad_pinstance:Option<AdPinstance>,
}

pub struct TTransaction {
	/// primary
	/// not nullable 
	pub ad_pinstance_id:f64,
	/// primary
	/// not nullable 
	pub m_transaction_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub c_project_id:Option<f64>,
	pub c_projectissue_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub m_attributesetinstance_id:f64,
	pub m_inout_id:Option<f64>,
	pub m_inoutline_id:Option<f64>,
	pub m_inventory_id:Option<f64>,
	pub m_inventoryline_id:Option<f64>,
	/// not nullable 
	pub m_locator_id:f64,
	pub m_movement_id:Option<f64>,
	pub m_movementline_id:Option<f64>,
	/// not nullable 
	pub m_product_id:f64,
	pub m_production_id:Option<f64>,
	pub m_productionline_id:Option<f64>,
	/// not nullable 
	pub movementdate:NaiveDateTime,
	/// defaults to: 0
	/// not nullable 
	pub movementqty:f64,
	/// not nullable 
	pub movementtype:String,
	pub search_inout_id:Option<f64>,
	pub search_invoice_id:Option<f64>,
	pub search_order_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub ad_pinstance_id_ad_pinstance:Option<AdPinstance>,
	/// has one
	pub m_transaction_id_m_transaction:Option<MTransaction>,
	/// has one
	pub m_locator_id_m_locator:Option<MLocator>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
	/// has one
	pub m_attributesetinstance_id_m_attributesetinstance:Option<MAttributesetinstance>,
	/// has one
	pub m_inoutline_id_m_inoutline:Option<MInoutline>,
	/// has one
	pub m_movementline_id_m_movementline:Option<MMovementline>,
	/// has one
	pub m_inventoryline_id_m_inventoryline:Option<MInventoryline>,
	/// has one
	pub m_productionline_id_m_productionline:Option<MProductionline>,
	/// has one
	pub c_projectissue_id_c_projectissue:Option<CProjectissue>,
}

pub struct TTrialbalance {
	/// primary
	/// not nullable 
	pub ad_pinstance_id:f64,
	/// primary
	/// not nullable 
	pub fact_acct_id:f64,
	pub a_asset_id:Option<f64>,
	pub account_id:Option<f64>,
	pub accountvalue:Option<String>,
	/// not nullable 
	pub ad_client_id:f64,
	pub ad_org_id:Option<f64>,
	pub ad_orgtrx_id:Option<f64>,
	pub ad_table_id:Option<f64>,
	/// not nullable 
	pub amtacctbalance:f64,
	/// not nullable 
	pub amtacctcr:f64,
	/// not nullable 
	pub amtacctdr:f64,
	pub amtsourcebalance:Option<f64>,
	pub amtsourcecr:Option<f64>,
	pub amtsourcedr:Option<f64>,
	/// not nullable 
	pub c_acctschema_id:f64,
	pub c_activity_id:Option<f64>,
	pub c_bpartner_id:Option<f64>,
	pub c_campaign_id:Option<f64>,
	pub c_currency_id:Option<f64>,
	pub c_locfrom_id:Option<f64>,
	pub c_locto_id:Option<f64>,
	pub c_period_id:Option<f64>,
	pub c_project_id:Option<f64>,
	pub c_salesregion_id:Option<f64>,
	pub c_tax_id:Option<f64>,
	pub c_uom_id:Option<f64>,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub dateacct:NaiveDateTime,
	pub datetrx:Option<NaiveDateTime>,
	pub description:Option<String>,
	pub gl_budget_id:Option<f64>,
	pub gl_category_id:Option<f64>,
	pub line_id:Option<f64>,
	pub m_locator_id:Option<f64>,
	pub m_product_id:Option<f64>,
	/// not nullable 
	pub postingtype:String,
	pub qty:Option<f64>,
	pub record_id:Option<f64>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub user1_id:Option<f64>,
	pub user2_id:Option<f64>,
	/// has one
	pub ad_pinstance_id_ad_pinstance:Option<AdPinstance>,
}

pub struct Test {
	/// primary
	/// not nullable 
	pub test_id:f64,
	pub account_acct:Option<f64>,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub binarydata:Option<Vec<u8>>,
	pub c_bpartner_id:Option<f64>,
	pub c_currency_id:Option<f64>,
	pub c_location_id:Option<f64>,
	pub c_payment_id:Option<f64>,
	pub c_uom_id:Option<f64>,
	pub characterdata:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub m_locator_id:Option<f64>,
	pub m_product_id:Option<f64>,
	/// not nullable 
	pub name:String,
	/// defaults to: 'N'::bpchar
	pub processed:Option<String>,
	pub processing:Option<String>,
	/// defaults to: 0
	pub t_amount:Option<f64>,
	pub t_date:Option<NaiveDateTime>,
	pub t_datetime:Option<NaiveDateTime>,
	pub t_integer:Option<f64>,
	/// defaults to: 0
	pub t_number:Option<f64>,
	/// defaults to: 0
	pub t_qty:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub account_acct_c_validcombination:Option<CValidcombination>,
}

pub struct Text {
	/// primary
	/// not nullable 
	pub text_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: NULL::numeric
	pub datatype_id:Option<f64>,
	/// defaults to: NULL::character varying
	pub description:Option<String>,
	/// defaults to: NULL::character varying
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub length:Option<f64>,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
}

pub struct Timestamp {
	/// primary
	/// not nullable 
	pub date_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: NULL::numeric
	pub datatype_id:Option<f64>,
	/// defaults to: NULL::character varying
	pub description:Option<String>,
	/// defaults to: NULL::character varying
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub length:Option<f64>,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
}

pub struct UBlacklistcheque {
	/// primary
	/// not nullable 
	pub u_blacklistcheque_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub bankname:String,
	/// not nullable 
	pub chequeno:String,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
}

pub struct UPosterminal {
	/// primary
	/// not nullable 
	pub u_posterminal_id:f64,
	/// unique
	/// defaults to: NULL::numeric
	/// not nullable 
	pub ad_client_id:f64,
	/// unique
	pub value:Option<String>,
	/// defaults to: NULL::numeric
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub autolock:String,
	/// not nullable 
	pub c_cashbook_id:f64,
	/// not nullable 
	pub c_cashbpartner_id:f64,
	pub c_templatebpartner_id:Option<f64>,
	pub card_bankaccount_id:Option<f64>,
	pub cardtransferbankaccount_id:Option<f64>,
	pub cardtransfercashbook_id:Option<f64>,
	pub cardtransfertype:Option<String>,
	/// not nullable 
	pub cashbooktransfertype:String,
	pub cashtransferbankaccount_id:Option<f64>,
	pub cashtransfercashbook_id:Option<f64>,
	pub check_bankaccount_id:Option<f64>,
	pub checktransferbankaccount_id:Option<f64>,
	pub checktransfercashbook_id:Option<f64>,
	pub checktransfertype:Option<String>,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	pub isactive:Option<String>,
	pub lastlocktime:Option<NaiveDateTime>,
	/// defaults to: 'N'::bpchar
	pub locked:Option<String>,
	pub locktime:Option<f64>,
	pub m_warehouse_id:Option<f64>,
	pub name:Option<String>,
	pub po_pricelist_id:Option<f64>,
	pub printername:Option<String>,
	pub salesrep_id:Option<f64>,
	pub so_pricelist_id:Option<f64>,
	pub unlockingtime:Option<NaiveDateTime>,
	/// not nullable 
	pub updated:NaiveDateTime,
	pub updatedby:Option<f64>,
}

pub struct URolemenu {
	/// primary
	/// not nullable 
	pub u_rolemenu_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_role_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:i32,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub u_webmenu_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:i32,
	/// has one
	pub ad_role_id_ad_role:Option<AdRole>,
	/// has one
	pub u_webmenu_id_u_webmenu:Option<UWebmenu>,
}

pub struct UWebProperties {
	/// primary
	/// not nullable 
	pub u_web_properties_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:i32,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub u_key:String,
	/// not nullable 
	pub u_value:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:i32,
}

pub struct UWebmenu {
	/// primary
	/// not nullable 
	pub u_webmenu_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub category:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub hassubmenu:String,
	pub help:Option<String>,
	pub imagelink:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub menulink:String,
	/// not nullable 
	pub module:String,
	/// not nullable 
	pub name:String,
	pub parentmenu_id:Option<f64>,
	pub position:Option<String>,
	pub sequence:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one, self referential
	pub parentmenu_id_u_webmenu:Option<Box<UWebmenu>>,
	/// has many
	pub u_rolemenu:Option<Vec<URolemenu>>,
	/// has many
	pub u_webmenu:Option<Vec<UWebmenu>>,
}

pub struct Users {
	/// primary
	/// defaults to: (0)::numeric
	/// not nullable 
	pub users_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: NULL::character varying
	pub description:Option<String>,
	/// defaults to: NULL::character varying
	pub email:Option<String>,
	/// defaults to: NULL::character varying
	pub firstname:Option<String>,
	/// defaults to: NULL::character varying
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: NULL::character varying
	pub lastname:Option<String>,
	/// defaults to: NULL::character varying
	pub middlename:Option<String>,
	/// not nullable 
	pub name:String,
	/// defaults to: NULL::character varying
	pub password:Option<String>,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
}

pub struct View {
	/// primary
	/// not nullable 
	pub view_id:f64,
	/// defaults to: NULL::numeric
	pub ad_class_id:Option<f64>,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: NULL::character varying
	pub description:Option<String>,
	/// defaults to: NULL::character varying
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
}

pub struct ViewAdClass {
	/// primary
	/// not nullable 
	pub view_ad_class_id:f64,
	/// defaults to: NULL::numeric
	pub ad_class_id:Option<f64>,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: NULL::character varying
	pub description:Option<String>,
	/// defaults to: NULL::character varying
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
	/// defaults to: NULL::numeric
	pub view_id:Option<f64>,
}

pub struct WAdvertisement {
	/// primary
	/// not nullable 
	pub w_advertisement_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_user_id:Option<f64>,
	pub adtext:Option<String>,
	/// not nullable 
	pub c_bpartner_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub help:Option<String>,
	pub imageurl:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isselfservice:String,
	/// not nullable 
	pub name:String,
	pub processing:Option<String>,
	/// not nullable 
	pub publishstatus:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub validfrom:Option<NaiveDateTime>,
	pub validto:Option<NaiveDateTime>,
	pub version:Option<f64>,
	pub w_clickcount_id:Option<f64>,
	pub w_countercount_id:Option<f64>,
	pub webparam1:Option<String>,
	pub webparam2:Option<String>,
	pub webparam3:Option<String>,
	pub webparam4:Option<String>,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
	/// has one
	pub w_clickcount_id_w_clickcount:Option<WClickcount>,
	/// has one
	pub w_countercount_id_w_countercount:Option<WCountercount>,
}

pub struct WBasket {
	/// primary
	/// not nullable 
	pub w_basket_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub ad_user_id:f64,
	pub c_bpartner_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub email:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub m_pricelist_id:Option<f64>,
	/// not nullable 
	pub session_id:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has one
	pub m_pricelist_id_m_pricelist:Option<MPricelist>,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
	/// has many
	pub w_basketline:Option<Vec<WBasketline>>,
}

pub struct WBasketline {
	/// primary
	/// not nullable 
	pub w_basketline_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// not nullable 
	pub description:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub line:f64,
	pub m_product_id:Option<f64>,
	/// defaults to: 0
	/// not nullable 
	pub price:f64,
	/// not nullable 
	pub product:String,
	/// defaults to: 0
	/// not nullable 
	pub qty:f64,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub w_basket_id:f64,
	/// has one
	pub w_basket_id_w_basket:Option<WBasket>,
	/// has one
	pub m_product_id_m_product:Option<MProduct>,
}

pub struct WClick {
	/// primary
	/// not nullable 
	pub w_click_id:f64,
	pub acceptlanguage:Option<String>,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_user_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub email:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	pub referrer:Option<String>,
	pub remote_addr:Option<String>,
	pub remote_host:Option<String>,
	pub targeturl:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub useragent:Option<String>,
	pub w_clickcount_id:Option<f64>,
	/// has one
	pub w_clickcount_id_w_clickcount:Option<WClickcount>,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
}

pub struct WClickcount {
	/// primary
	/// not nullable 
	pub w_clickcount_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub c_bpartner_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub targeturl:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has many
	pub w_advertisement:Option<Vec<WAdvertisement>>,
	/// has many
	pub w_click:Option<Vec<WClick>>,
}

pub struct WCounter {
	/// primary
	/// not nullable 
	pub w_counter_id:f64,
	pub acceptlanguage:Option<String>,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub ad_user_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub email:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub pageurl:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub processed:String,
	pub referrer:Option<String>,
	pub remote_addr:Option<String>,
	pub remote_host:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub useragent:Option<String>,
	pub w_countercount_id:Option<f64>,
	/// has one
	pub w_countercount_id_w_countercount:Option<WCountercount>,
	/// has one
	pub ad_user_id_ad_user:Option<AdUser>,
}

pub struct WCountercount {
	/// primary
	/// not nullable 
	pub w_countercount_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub c_bpartner_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub pageurl:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub c_bpartner_id_c_bpartner:Option<CBpartner>,
	/// has many
	pub w_advertisement:Option<Vec<WAdvertisement>>,
	/// has many
	pub w_counter:Option<Vec<WCounter>>,
}

pub struct WMailmsg {
	/// primary
	/// not nullable 
	pub w_mailmsg_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// not nullable 
	pub mailmsgtype:String,
	/// not nullable 
	pub message:String,
	pub message2:Option<String>,
	pub message3:Option<String>,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub subject:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub w_store_id:f64,
	/// has one
	pub w_store_id_w_store:Option<WStore>,
	/// has many
	pub ad_usermail:Option<Vec<AdUsermail>>,
	/// has many
	pub w_mailmsg_trl:Option<Vec<WMailmsgTrl>>,
}

pub struct WMailmsgTrl {
	/// primary
	/// not nullable 
	pub w_mailmsg_id:f64,
	/// primary
	/// not nullable 
	pub ad_language:String,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub istranslated:String,
	/// not nullable 
	pub message:String,
	pub message2:Option<String>,
	pub message3:Option<String>,
	/// not nullable 
	pub subject:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// has one
	pub w_mailmsg_id_w_mailmsg:Option<WMailmsg>,
	/// has one
	pub ad_language_ad_language:Option<AdLanguage>,
}

pub struct WStore {
	/// primary
	/// not nullable 
	pub w_store_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	pub c_paymentterm_id:Option<f64>,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub description:Option<String>,
	pub emailfooter:Option<String>,
	pub emailheader:Option<String>,
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub isdefault:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub ismenuassets:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub ismenucontact:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub ismenuinterests:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub ismenuinvoices:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub ismenuorders:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub ismenupayments:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub ismenuregistrations:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub ismenurequests:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub ismenurfqs:String,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub ismenushipments:String,
	/// not nullable 
	pub m_pricelist_id:f64,
	/// not nullable 
	pub m_warehouse_id:f64,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub salesrep_id:f64,
	pub stylesheet:Option<String>,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// defaults to: 'http://localhost'::character varying
	/// not nullable 
	pub url:String,
	/// not nullable 
	pub webcontext:String,
	pub webinfo:Option<String>,
	pub weborderemail:Option<String>,
	pub webparam1:Option<String>,
	pub webparam2:Option<String>,
	pub webparam3:Option<String>,
	pub webparam4:Option<String>,
	pub webparam5:Option<String>,
	pub webparam6:Option<String>,
	pub wstoreemail:Option<String>,
	pub wstoreuser:Option<String>,
	pub wstoreuserpw:Option<String>,
	/// has one
	pub ad_client_id_ad_client:Option<AdClient>,
	/// has one
	pub salesrep_id_ad_user:Option<AdUser>,
	/// has one
	pub m_warehouse_id_m_warehouse:Option<MWarehouse>,
	/// has one
	pub m_pricelist_id_m_pricelist:Option<MPricelist>,
	/// has one
	pub c_paymentterm_id_c_paymentterm:Option<CPaymentterm>,
	/// has many
	pub w_mailmsg:Option<Vec<WMailmsg>>,
	/// has many
	pub w_store_trl:Option<Vec<WStoreTrl>>,
}

pub struct WStoreTrl {
	/// primary
	/// not nullable 
	pub w_store_id:f64,
	/// primary
	/// not nullable 
	pub ad_language:String,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// defaults to: now()
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	pub emailfooter:Option<String>,
	pub emailheader:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	/// defaults to: 'N'::bpchar
	/// not nullable 
	pub istranslated:String,
	/// defaults to: now()
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	pub webinfo:Option<String>,
	pub webparam1:Option<String>,
	pub webparam2:Option<String>,
	pub webparam3:Option<String>,
	pub webparam4:Option<String>,
	pub webparam5:Option<String>,
	pub webparam6:Option<String>,
	/// has one
	pub w_store_id_w_store:Option<WStore>,
	/// has one
	pub ad_language_ad_language:Option<AdLanguage>,
}

pub struct Yesno {
	/// primary
	/// not nullable 
	pub yesno_id:f64,
	/// not nullable 
	pub ad_client_id:f64,
	/// not nullable 
	pub ad_org_id:f64,
	/// not nullable 
	pub created:NaiveDateTime,
	/// not nullable 
	pub createdby:f64,
	/// defaults to: NULL::numeric
	pub datatype_id:Option<f64>,
	/// defaults to: NULL::character varying
	pub description:Option<String>,
	/// defaults to: NULL::character varying
	pub help:Option<String>,
	/// defaults to: 'Y'::bpchar
	/// not nullable 
	pub isactive:String,
	pub length:Option<f64>,
	/// not nullable 
	pub name:String,
	/// not nullable 
	pub updated:NaiveDateTime,
	/// not nullable 
	pub updatedby:f64,
	/// not nullable 
	pub value:String,
}