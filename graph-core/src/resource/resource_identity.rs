use heck::{ToLowerCamelCase, ToSnakeCase, ToUpperCamelCase};
use std::fmt::Display;
use std::str::FromStr;

/// Comprises both top level and second level resources.
/// These are not generated from OpenApi, except for top level resources,
/// and mostly consist of Apis that the project currently has generated.
#[derive(
    Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
#[non_exhaustive]
pub enum ResourceIdentity {
    AccessPackageAssignmentApprovals,
    AccessPackages,
    AccessReviews,
    AccessReviewsDefinitions,
    AccessReviewsDefinitionsInstances,
    AccessReviewsDefinitionsInstancesStages,
    Activities,
    Admin,
    AdministrativeUnits,
    AgreementAcceptances,
    Agreements,
    AllChannels,
    AndroidManagedAppProtections,
    AppCatalogs,
    AppConsent,
    Application,
    Applications,
    ApplicationTemplates,
    Appointments,
    AppRoleAssignments,
    AssignmentPolicies,
    AssignmentRequests,
    Assignments,
    AuditLogs,
    Authentication,
    AuthenticationMethodConfigurations,
    AuthenticationMethodsPolicy,
    Batch, // Specifically for $batch requests.
    BookingBusinesses,
    Branding,
    Buckets,
    CalendarGroups,
    Calendars,
    CalendarView,
    CallRecords,
    CallRecordsSessions,
    Calls,
    CertificateBasedAuthConfiguration,
    Channels,
    Chats,
    ChatsAndChannelsMessages,
    ChatsMessages,
    ChatsMessagesReplies,
    ChildFolders,
    Communications,
    Compliance,
    ConnectedOrganizations,
    ConnectedOrganizationsExternalSponsors,
    ConnectedOrganizationsInternalSponsors,
    Connections,
    ContactFolders,
    Contacts,
    ContentTypes,
    Contracts,
    Conversations,
    CreatedByUser,
    CreatedObjects,
    Custom,
    Customers,
    CustomQuestions,
    DataPolicyOperations,
    DefaultCalendar,
    DefaultManagedAppProtections,
    DeletedItems,
    DeletedTeams,
    DeviceAppManagement,
    DeviceCompliancePolicies,
    DeviceCompliancePolicySettingStateSummaries,
    DeviceConfigurations,
    DeviceEnrollmentConfigurations,
    DeviceManagement,
    DeviceManagementManagedDevices,
    DeviceManagementReports,
    Devices,
    DevicesRegisteredOwners,
    DevicesRegisteredUsers,
    Directory,
    DirectoryMembers,
    DirectoryObjects,
    DirectoryRoles,
    DirectoryRoleTemplates,
    DirectReports,
    DomainDnsRecords,
    Domains,
    Drive,
    Drives,
    DrivesItems,
    DrivesList,
    DrivesListContentTypes,
    Education,
    EducationAssignments,
    EducationAssignmentsSubmissions,
    EducationClasses,
    EducationMe,
    EducationSchools,
    EducationUsers,
    EntitlementManagement,
    EntitlementManagementAssignments,
    EntitlementManagementCatalogs,
    Events,
    EventsInstances,
    ExtendedProperties,
    Extensions,
    External,
    FollowedSites,
    GroupLifecyclePolicies,
    GroupSettings,
    GroupSettingTemplates,
    Groups,
    GroupsOwners,
    GroupsTeam,
    HistoryItems,
    Identity,
    IdentityGovernance,
    IdentityProtection,
    IdentityProviders,
    IncomingChannels,
    InferenceClassification,
    InformationProtection,
    Insights,
    Invitations,
    IosManagedAppProtections,
    JoinedTeams,
    LastModifiedByUser,
    LicenseDetails,
    List,
    Localizations,
    MailboxSettings,
    MailFolders,
    ManagedAppPolicies,
    ManagedAppRegistrations,
    ManagedAppRegistrationsAppliedPolicies,
    ManagedAppRegistrationsIntendedPolicies,
    ManagedAppStatuses,
    ManagedEBooks,
    ManagedEBooksDeviceStates,
    ManagedEBooksUserStateSummary,
    MdmWindowsInformationProtectionPolicies,
    #[default]
    Me,
    MemberOf,
    MembersWithLicenseErrors,
    MobileAppCategories,
    MobileAppConfigurations,
    MobileApps,
    Oauth2PermissionGrants,
    Onenote,
    OnenoteNotebooks,
    OnenotePages,
    OnenoteSectionGroups,
    OnenoteSections,
    OnlineMeetings,
    Organization,
    OrgContacts,
    Outlook,
    OwnedDevices,
    OwnedObjects,
    ParentNotebook,
    ParentSection,
    ParentSectionGroup,
    People,
    PermissionGrants,
    Photos,
    Places,
    Planner,
    PlannerTasks,
    Plans,
    Policies,
    Presence,
    PrimaryChannel,
    Print,
    Privacy,
    RegisteredDevices,
    Reports,
    RoleDefinitions,
    RoleManagement,
    Schedule,
    SchemaExtensions,
    ScopedRoleMemberOf,
    ScopedRoleMemberships,
    Search,
    Security,
    ServicePrincipals,
    ServicePrincipalsOwners,
    Services,
    Settings,
    SharedWithTeams,
    Shares,
    Sites,
    SitesContentTypes,
    SitesItems,
    SitesItemsVersions,
    SitesLists,
    Solutions,
    StaffMembers,
    SubscribedSkus,
    Subscriptions,
    Tabs,
    TargetedManagedAppConfigurations,
    Tasks,
    Teams,
    TeamsMembers,
    TeamsPrimaryChannelTabs,
    TeamsTags,
    TeamsTemplates,
    Teamwork,
    TermStore,
    TermStoreGroups,
    TermStoreSets,
    TermStoreSetsChildren,
    TermStoreSetsParentGroup,
    TermStoreSetsRelations,
    TermStoreSetsTerms,
    TermStores,
    TermsAndConditions,
    Threads,
    ThreadsPosts,
    Todo,
    TodoLists,
    TodoListsTasks,
    TransitiveMemberOf,
    TransitiveMembers,
    TroubleshootingEvents,
    Users,
    UsersAttachments,
    UsersManagedDevices,
    UsersMessages,
    VirtualEvents,
    VirtualEventsEvents,
    VirtualEventsSessions,
    VirtualEventsWebinars,
    VppTokens,
    WindowsAutopilotDeviceIdentities,
    WindowsInformationProtectionPolicies,
    Workbook,
    WorkbookFunctions,
    WorkbookTables,
    WorkbookTablesColumns,
    WorkbookTablesRows,
    Worksheets,
    WorksheetsCharts,
    WorksheetsChartsAxes,
    WorksheetsChartsAxesCategoryAxis,
    WorksheetsChartsAxesSeriesAxis,
    WorksheetsChartsAxesValueAxis,
    WorksheetsChartsDataLabels,
    WorksheetsChartsFormat,
    WorksheetsChartsLegend,
    WorksheetsChartsSeries,
    WorksheetsChartsTitle,
}

impl Display for ResourceIdentity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            ResourceIdentity::AccessPackages => "accessPackages".to_string(),
            ResourceIdentity::AccessReviewsDefinitions => "definitions".to_string(),
            ResourceIdentity::AccessReviewsDefinitionsInstances => "instances".to_string(),
            ResourceIdentity::AccessReviewsDefinitionsInstancesStages => "stages".to_string(),
            ResourceIdentity::DeviceManagementManagedDevices
            | ResourceIdentity::UsersManagedDevices => "managedDevices".to_string(),
            ResourceIdentity::DeviceManagementReports => "reports".into(),
            ResourceIdentity::EntitlementManagementAssignments => "assignments".to_string(),
            ResourceIdentity::EntitlementManagementCatalogs => "catalogs".to_string(),
            ResourceIdentity::PrimaryChannel => "primaryChannel".to_string(),
            ResourceIdentity::TeamsTags => "tags".to_string(),
            ResourceIdentity::DirectoryMembers | ResourceIdentity::TeamsMembers => {
                "members".to_string()
            }
            ResourceIdentity::SharedWithTeams => "sharedWithTeams".to_string(),
            ResourceIdentity::DrivesList => "list".to_string(),
            ResourceIdentity::DrivesItems | ResourceIdentity::SitesItems => "items".to_string(),
            ResourceIdentity::SitesItemsVersions => "versions".to_string(),
            ResourceIdentity::SitesLists => "lists".to_string(),
            ResourceIdentity::DrivesListContentTypes | ResourceIdentity::SitesContentTypes => {
                "contentTypes".to_string()
            }
            ResourceIdentity::Conversations => "conversations".into(),
            ResourceIdentity::GroupsOwners | ResourceIdentity::ServicePrincipalsOwners => {
                "owners".into()
            }
            ResourceIdentity::GroupsTeam => "team".into(),
            ResourceIdentity::ThreadsPosts => "posts".into(),
            ResourceIdentity::Threads => "threads".into(),
            ResourceIdentity::Activities => "activities".to_string(),
            ResourceIdentity::AgreementAcceptances => "agreementAcceptances".to_string(),
            ResourceIdentity::AppRoleAssignments => "appRoleAssignments".to_string(),
            ResourceIdentity::Authentication => "authentication".to_string(),
            ResourceIdentity::DefaultCalendar => "calendar".to_string(),
            ResourceIdentity::CalendarGroups => "calendarGroups".to_string(),
            ResourceIdentity::CalendarView => "calendarView".to_string(),
            ResourceIdentity::Calendars => "calendars".to_string(),
            ResourceIdentity::Chats => "chats".to_string(),
            ResourceIdentity::ChatsMessages => "messages".to_string(),
            ResourceIdentity::ChatsMessagesReplies => "replies".to_string(),
            ResourceIdentity::ContactFolders => "contactFolders".to_string(),
            ResourceIdentity::Contacts => "contacts".to_string(),
            ResourceIdentity::CreatedObjects => "createdObjects".to_string(),
            ResourceIdentity::DirectReports => "directReports".to_string(),
            ResourceIdentity::Drives => "drives".to_string(),
            ResourceIdentity::Events => "events".to_string(),
            ResourceIdentity::Extensions => "extensions".to_string(),
            ResourceIdentity::FollowedSites => "followedSites".to_string(),
            ResourceIdentity::InferenceClassification => "inferenceClassification".to_string(),
            ResourceIdentity::Insights => "insights".to_string(),
            ResourceIdentity::JoinedTeams => "joinedTeams".to_string(),
            ResourceIdentity::LicenseDetails => "licenseDetails".to_string(),
            ResourceIdentity::MailFolders => "mailFolders".to_string(),
            ResourceIdentity::ManagedAppRegistrations => "managedAppRegistrations".to_string(),
            ResourceIdentity::ManagedAppRegistrationsAppliedPolicies => {
                "appliedPolicies".to_string()
            }
            ResourceIdentity::ManagedAppRegistrationsIntendedPolicies => {
                "intendedPolicies".to_string()
            }
            ResourceIdentity::ManagedEBooksDeviceStates => "deviceStates".to_string(),
            ResourceIdentity::ManagedEBooksUserStateSummary => "userStateSummary".to_string(),
            ResourceIdentity::MemberOf => "memberOf".to_string(),
            ResourceIdentity::Oauth2PermissionGrants => "oauth2PermissionGrants".to_string(),
            ResourceIdentity::Onenote => "onenote".to_string(),
            ResourceIdentity::OnlineMeetings => "onlineMeetings".to_string(),
            ResourceIdentity::Outlook => "outlook".to_string(),
            ResourceIdentity::OwnedDevices => "ownedDevices".to_string(),
            ResourceIdentity::OwnedObjects => "ownedObjects".to_string(),
            ResourceIdentity::People => "people".to_string(),
            ResourceIdentity::Photos => "photos".to_string(),
            ResourceIdentity::Planner => "planner".to_string(),
            ResourceIdentity::Presence => "presence".to_string(),
            ResourceIdentity::RegisteredDevices => "registeredDevices".to_string(),
            ResourceIdentity::ScopedRoleMemberOf => "scopedRoleMemberOf".to_string(),
            ResourceIdentity::Teamwork => "teamwork".to_string(),
            ResourceIdentity::Todo => "todo".to_string(),
            ResourceIdentity::TransitiveMemberOf => "transitiveMemberOf".to_string(),

            ResourceIdentity::ConnectedOrganizationsExternalSponsors => {
                "externalSponsors".to_string()
            }
            ResourceIdentity::ConnectedOrganizationsInternalSponsors => {
                "internalSponsors".to_string()
            }
            ResourceIdentity::CallRecordsSessions => "sessions".to_string(),
            ResourceIdentity::EducationAssignmentsSubmissions => "submissions".to_string(),
            ResourceIdentity::EducationAssignments => "assignments".to_string(),
            ResourceIdentity::EducationClasses => "classes".to_string(),
            ResourceIdentity::EducationMe => "me".to_string(),
            ResourceIdentity::EducationUsers => "users".to_string(),
            ResourceIdentity::EducationSchools => "schools".to_string(),
            ResourceIdentity::TodoLists => "lists".to_string(),
            ResourceIdentity::TodoListsTasks => "tasks".to_string(),
            ResourceIdentity::UsersMessages => "messages".into(),
            ResourceIdentity::UsersAttachments => "attachments".into(),
            ResourceIdentity::EventsInstances => "instances".into(),
            ResourceIdentity::PlannerTasks => "tasks".into(),
            ResourceIdentity::OnenoteSections => "sections".into(),
            ResourceIdentity::OnenoteSectionGroups => "sectionGroups".into(),
            ResourceIdentity::OnenoteNotebooks => "notebooks".into(),
            ResourceIdentity::OnenotePages => "pages".into(),
            ResourceIdentity::TermStoreSets => "sets".into(),
            ResourceIdentity::TermStoreGroups => "groups".into(),
            ResourceIdentity::TermStoreSetsChildren => "children".into(),
            ResourceIdentity::TermStoreSetsParentGroup => "parentGroup".into(),
            ResourceIdentity::TermStoreSetsRelations => "relations".into(),
            ResourceIdentity::TermStoreSetsTerms => "terms".into(),
            ResourceIdentity::VirtualEvents => "virtualEvents".into(),
            ResourceIdentity::VirtualEventsEvents => "events".into(),
            ResourceIdentity::VirtualEventsSessions => "sessions".into(),
            ResourceIdentity::VirtualEventsWebinars => "webinars".into(),
            ResourceIdentity::WorkbookFunctions => "functions".into(),
            ResourceIdentity::WorkbookTables => "tables".into(),
            ResourceIdentity::WorkbookTablesColumns => "columns".into(),
            ResourceIdentity::WorkbookTablesRows => "rows".into(),
            ResourceIdentity::WorksheetsCharts => "charts".into(),
            ResourceIdentity::WorksheetsChartsAxes => "axes".into(),
            ResourceIdentity::WorksheetsChartsLegend => "legend".into(),
            ResourceIdentity::WorksheetsChartsSeries => "series".into(),
            ResourceIdentity::WorksheetsChartsFormat => "format".into(),
            ResourceIdentity::WorksheetsChartsTitle => "title".into(),
            ResourceIdentity::WorksheetsChartsDataLabels => "dataLabels".into(),
            ResourceIdentity::WorksheetsChartsAxesCategoryAxis => "categoryAxis".into(),
            ResourceIdentity::WorksheetsChartsAxesSeriesAxis => "seriesAxis".into(),
            ResourceIdentity::WorksheetsChartsAxesValueAxis => "valueAxis".into(),
            ResourceIdentity::DevicesRegisteredOwners => "registeredOwners".into(),
            ResourceIdentity::DevicesRegisteredUsers => "registeredUsers".into(),
            ResourceIdentity::Custom => "".into(),

            _ => self.as_ref().to_lower_camel_case(),
        };
        write!(f, "{}", str)
    }
}

impl ResourceIdentity {
    pub fn enum_string(&self) -> String {
        format!("ResourceIdentity::{self:#?}")
    }

    pub fn to_path_start(&self) -> String {
        format!("/{}", self)
    }

    pub fn replace(&self, from: &str, to: &str) -> String {
        self.as_ref().replace(from, to)
    }

    pub fn exact_camel_case(&self) -> String {
        self.as_ref().to_lower_camel_case()
    }

    pub fn exact_pascal_case(&self) -> String {
        self.as_ref().to_upper_camel_case()
    }

    pub fn exact_snake_case(&self) -> String {
        self.as_ref().to_snake_case()
    }
}

/// Top level resources are the names for the first or beginning part of a URI path.
/// These are generated from the OpenApi config.
#[derive(
    Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum TopLevelResource {
    Admin,
    AgreementAcceptances,
    Agreements,
    AppCatalogs,
    ApplicationTemplates,
    Applications,
    AuditLogs,
    AuthenticationMethodConfigurations,
    AuthenticationMethodsPolicy,
    Branding,
    CertificateBasedAuthConfiguration,
    Chats,
    Communications,
    Compliance,
    Connections,
    Contacts,
    Contracts,
    DataPolicyOperations,
    DeviceAppManagement,
    DeviceManagement,
    Devices,
    Directory,
    DirectoryObjects,
    DirectoryRoleTemplates,
    DirectoryRoles,
    DomainDnsRecords,
    Domains,
    Drive,
    Drives,
    Education,
    External,
    GroupLifecyclePolicies,
    GroupSettingTemplates,
    GroupSettings,
    Groups,
    Identity,
    IdentityGovernance,
    IdentityProtection,
    IdentityProviders,
    InformationProtection,
    Invitations,
    Localizations,
    #[default]
    Me,
    Oauth2PermissionGrants,
    Organization,
    PermissionGrants,
    Places,
    Planner,
    Policies,
    Print,
    Privacy,
    Reports,
    RoleManagement,
    SchemaExtensions,
    ScopedRoleMemberships,
    Search,
    Security,
    ServicePrincipals,
    Shares,
    Sites,
    Solutions,
    SubscribedSkus,
    Subscriptions,
    Teams,
    TeamsTemplates,
    Teamwork,
    Users,
}

impl Display for TopLevelResource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref().to_lower_camel_case())
    }
}

impl AsRef<str> for ResourceIdentity {
    fn as_ref(&self) -> &str {
        match self {
            ResourceIdentity::AccessPackageAssignmentApprovals => {
                "AccessPackageAssignmentApprovals"
            }
            ResourceIdentity::AccessPackages => "AccessPackages",
            ResourceIdentity::AccessReviews => "AccessReviews",
            ResourceIdentity::AccessReviewsDefinitions => "AccessReviewsDefinitions",
            ResourceIdentity::AccessReviewsDefinitionsInstances => {
                "AccessReviewsDefinitionsInstances"
            }
            ResourceIdentity::AccessReviewsDefinitionsInstancesStages => {
                "AccessReviewsDefinitionsInstancesStages"
            }
            ResourceIdentity::Activities => "Activities",
            ResourceIdentity::Admin => "Admin",
            ResourceIdentity::AdministrativeUnits => "AdministrativeUnits",
            ResourceIdentity::AgreementAcceptances => "AgreementAcceptances",
            ResourceIdentity::Agreements => "Agreements",
            ResourceIdentity::AllChannels => "AllChannels",
            ResourceIdentity::AndroidManagedAppProtections => "AndroidManagedAppProtections",
            ResourceIdentity::AppCatalogs => "AppCatalogs",
            ResourceIdentity::AppConsent => "AppConsent",
            ResourceIdentity::Application => "Application",
            ResourceIdentity::Applications => "Applications",
            ResourceIdentity::ApplicationTemplates => "ApplicationTemplates",
            ResourceIdentity::Appointments => "Appointments",
            ResourceIdentity::AppRoleAssignments => "AppRoleAssignments",
            ResourceIdentity::AssignmentPolicies => "AssignmentPolicies",
            ResourceIdentity::AssignmentRequests => "AssignmentRequests",
            ResourceIdentity::Assignments => "Assignments",
            ResourceIdentity::AuditLogs => "AuditLogs",
            ResourceIdentity::Authentication => "Authentication",
            ResourceIdentity::AuthenticationMethodConfigurations => {
                "AuthenticationMethodConfigurations"
            }
            ResourceIdentity::AuthenticationMethodsPolicy => "AuthenticationMethodsPolicy",
            ResourceIdentity::Batch => "Batch",
            ResourceIdentity::BookingBusinesses => "BookingBusinesses",
            ResourceIdentity::Branding => "Branding",
            ResourceIdentity::Buckets => "Buckets",
            ResourceIdentity::CalendarGroups => "CalendarGroups",
            ResourceIdentity::Calendars => "Calendars",
            ResourceIdentity::CalendarView => "CalendarView",
            ResourceIdentity::CallRecords => "CallRecords",
            ResourceIdentity::CallRecordsSessions => "CallRecordsSessions",
            ResourceIdentity::Calls => "Calls",
            ResourceIdentity::CertificateBasedAuthConfiguration => {
                "CertificateBasedAuthConfiguration"
            }
            ResourceIdentity::Channels => "Channels",
            ResourceIdentity::Chats => "Chats",
            ResourceIdentity::ChatsAndChannelsMessages => "ChatsAndChannelsMessages",
            ResourceIdentity::ChatsMessages => "ChatsMessages",
            ResourceIdentity::ChatsMessagesReplies => "ChatsMessagesReplies",
            ResourceIdentity::ChildFolders => "ChildFolders",
            ResourceIdentity::Communications => "Communications",
            ResourceIdentity::Compliance => "Compliance",
            ResourceIdentity::ConnectedOrganizations => "ConnectedOrganizations",
            ResourceIdentity::ConnectedOrganizationsExternalSponsors => {
                "ConnectedOrganizationsExternalSponsors"
            }
            ResourceIdentity::ConnectedOrganizationsInternalSponsors => {
                "ConnectedOrganizationsInternalSponsors"
            }
            ResourceIdentity::Connections => "Connections",
            ResourceIdentity::ContactFolders => "ContactFolders",
            ResourceIdentity::Contacts => "Contacts",
            ResourceIdentity::ContentTypes => "ContentTypes",
            ResourceIdentity::Contracts => "Contracts",
            ResourceIdentity::Conversations => "Conversations",
            ResourceIdentity::CreatedByUser => "CreatedByUser",
            ResourceIdentity::CreatedObjects => "CreatedObjects",
            ResourceIdentity::Custom => "Custom",
            ResourceIdentity::Customers => "Customers",
            ResourceIdentity::CustomQuestions => "CustomQuestions",
            ResourceIdentity::DataPolicyOperations => "DataPolicyOperations",
            ResourceIdentity::DefaultCalendar => "DefaultCalendar",
            ResourceIdentity::DefaultManagedAppProtections => "DefaultManagedAppProtections",
            ResourceIdentity::DeletedItems => "DeletedItems",
            ResourceIdentity::DeletedTeams => "DeletedTeams",
            ResourceIdentity::DeviceAppManagement => "DeviceAppManagement",
            ResourceIdentity::DeviceCompliancePolicies => "DeviceCompliancePolicies",
            ResourceIdentity::DeviceCompliancePolicySettingStateSummaries => {
                "DeviceCompliancePolicySettingStateSummaries"
            }
            ResourceIdentity::DeviceConfigurations => "DeviceConfigurations",
            ResourceIdentity::DeviceEnrollmentConfigurations => "DeviceEnrollmentConfigurations",
            ResourceIdentity::DeviceManagement => "DeviceManagement",
            ResourceIdentity::DeviceManagementManagedDevices => "DeviceManagementManagedDevices",
            ResourceIdentity::DeviceManagementReports => "DeviceManagementReports",
            ResourceIdentity::Devices => "Devices",
            ResourceIdentity::DevicesRegisteredOwners => "DevicesRegisteredOwners",
            ResourceIdentity::DevicesRegisteredUsers => "DevicesRegisteredUsers",
            ResourceIdentity::Directory => "Directory",
            ResourceIdentity::DirectoryMembers => "DirectoryMembers",
            ResourceIdentity::DirectoryObjects => "DirectoryObjects",
            ResourceIdentity::DirectoryRoles => "DirectoryRoles",
            ResourceIdentity::DirectoryRoleTemplates => "DirectoryRoleTemplates",
            ResourceIdentity::DirectReports => "DirectReports",
            ResourceIdentity::DomainDnsRecords => "DomainDnsRecords",
            ResourceIdentity::Domains => "Domains",
            ResourceIdentity::Drive => "Drive",
            ResourceIdentity::Drives => "Drives",
            ResourceIdentity::DrivesItems => "DrivesItems",
            ResourceIdentity::DrivesList => "DrivesList",
            ResourceIdentity::DrivesListContentTypes => "DrivesListContentTypes",
            ResourceIdentity::Education => "Education",
            ResourceIdentity::EducationAssignments => "EducationAssignments",
            ResourceIdentity::EducationAssignmentsSubmissions => "EducationAssignmentsSubmissions",
            ResourceIdentity::EducationClasses => "EducationClasses",
            ResourceIdentity::EducationMe => "EducationMe",
            ResourceIdentity::EducationSchools => "EducationSchools",
            ResourceIdentity::EducationUsers => "EducationUsers",
            ResourceIdentity::EntitlementManagement => "EntitlementManagement",
            ResourceIdentity::EntitlementManagementAssignments => {
                "EntitlementManagementAssignments"
            }
            ResourceIdentity::EntitlementManagementCatalogs => "EntitlementManagementCatalogs",
            ResourceIdentity::Events => "Events",
            ResourceIdentity::EventsInstances => "EventsInstances",
            ResourceIdentity::ExtendedProperties => "ExtendedProperties",
            ResourceIdentity::Extensions => "Extensions",
            ResourceIdentity::External => "External",
            ResourceIdentity::FollowedSites => "FollowedSites",
            ResourceIdentity::GroupLifecyclePolicies => "GroupLifecyclePolicies",
            ResourceIdentity::GroupSettings => "GroupSettings",
            ResourceIdentity::GroupSettingTemplates => "GroupSettingTemplates",
            ResourceIdentity::Groups => "Groups",
            ResourceIdentity::GroupsOwners => "GroupsOwners",
            ResourceIdentity::GroupsTeam => "GroupsTeam",
            ResourceIdentity::HistoryItems => "HistoryItems",
            ResourceIdentity::Identity => "Identity",
            ResourceIdentity::IdentityGovernance => "IdentityGovernance",
            ResourceIdentity::IdentityProtection => "IdentityProtection",
            ResourceIdentity::IdentityProviders => "IdentityProviders",
            ResourceIdentity::IncomingChannels => "IncomingChannels",
            ResourceIdentity::InferenceClassification => "InferenceClassification",
            ResourceIdentity::InformationProtection => "InformationProtection",
            ResourceIdentity::Insights => "Insights",
            ResourceIdentity::Invitations => "Invitations",
            ResourceIdentity::IosManagedAppProtections => "IosManagedAppProtections",
            ResourceIdentity::JoinedTeams => "JoinedTeams",
            ResourceIdentity::LastModifiedByUser => "LastModifiedByUser",
            ResourceIdentity::LicenseDetails => "LicenseDetails",
            ResourceIdentity::List => "List",
            ResourceIdentity::Localizations => "Localizations",
            ResourceIdentity::MailboxSettings => "MailboxSettings",
            ResourceIdentity::MailFolders => "MailFolders",
            ResourceIdentity::ManagedAppPolicies => "ManagedAppPolicies",
            ResourceIdentity::ManagedAppRegistrations => "ManagedAppRegistrations",
            ResourceIdentity::ManagedAppRegistrationsAppliedPolicies => {
                "ManagedAppRegistrationsAppliedPolicies"
            }
            ResourceIdentity::ManagedAppRegistrationsIntendedPolicies => {
                "ManagedAppRegistrationsIntendedPolicies"
            }
            ResourceIdentity::ManagedAppStatuses => "ManagedAppStatuses",
            ResourceIdentity::ManagedEBooks => "ManagedEBooks",
            ResourceIdentity::ManagedEBooksDeviceStates => "ManagedEBooksDeviceStates",
            ResourceIdentity::ManagedEBooksUserStateSummary => "ManagedEBooksUserStateSummary",
            ResourceIdentity::MdmWindowsInformationProtectionPolicies => {
                "MdmWindowsInformationProtectionPolicies"
            }
            ResourceIdentity::Me => "Me",
            ResourceIdentity::MemberOf => "MemberOf",
            ResourceIdentity::MembersWithLicenseErrors => "MembersWithLicenseErrors",
            ResourceIdentity::MobileAppCategories => "MobileAppCategories",
            ResourceIdentity::MobileAppConfigurations => "MobileAppConfigurations",
            ResourceIdentity::MobileApps => "MobileApps",
            ResourceIdentity::Oauth2PermissionGrants => "Oauth2PermissionGrants",
            ResourceIdentity::Onenote => "Onenote",
            ResourceIdentity::OnenoteNotebooks => "OnenoteNotebooks",
            ResourceIdentity::OnenotePages => "OnenotePages",
            ResourceIdentity::OnenoteSectionGroups => "OnenoteSectionGroups",
            ResourceIdentity::OnenoteSections => "OnenoteSections",
            ResourceIdentity::OnlineMeetings => "OnlineMeetings",
            ResourceIdentity::Organization => "Organization",
            ResourceIdentity::OrgContacts => "OrgContacts",
            ResourceIdentity::Outlook => "Outlook",
            ResourceIdentity::OwnedDevices => "OwnedDevices",
            ResourceIdentity::OwnedObjects => "OwnedObjects",
            ResourceIdentity::ParentNotebook => "ParentNotebook",
            ResourceIdentity::ParentSection => "ParentSection",
            ResourceIdentity::ParentSectionGroup => "ParentSectionGroup",
            ResourceIdentity::People => "People",
            ResourceIdentity::PermissionGrants => "PermissionGrants",
            ResourceIdentity::Photos => "Photos",
            ResourceIdentity::Places => "Places",
            ResourceIdentity::Planner => "Planner",
            ResourceIdentity::PlannerTasks => "PlannerTasks",
            ResourceIdentity::Plans => "Plans",
            ResourceIdentity::Policies => "Policies",
            ResourceIdentity::Presence => "Presence",
            ResourceIdentity::PrimaryChannel => "PrimaryChannel",
            ResourceIdentity::Print => "Print",
            ResourceIdentity::Privacy => "Privacy",
            ResourceIdentity::RegisteredDevices => "RegisteredDevices",
            ResourceIdentity::Reports => "Reports",
            ResourceIdentity::RoleDefinitions => "RoleDefinitions",
            ResourceIdentity::RoleManagement => "RoleManagement",
            ResourceIdentity::Schedule => "Schedule",
            ResourceIdentity::SchemaExtensions => "SchemaExtensions",
            ResourceIdentity::ScopedRoleMemberOf => "ScopedRoleMemberOf",
            ResourceIdentity::ScopedRoleMemberships => "ScopedRoleMemberships",
            ResourceIdentity::Search => "Search",
            ResourceIdentity::Security => "Security",
            ResourceIdentity::ServicePrincipals => "ServicePrincipals",
            ResourceIdentity::ServicePrincipalsOwners => "ServicePrincipalsOwners",
            ResourceIdentity::Services => "Services",
            ResourceIdentity::Settings => "Settings",
            ResourceIdentity::SharedWithTeams => "SharedWithTeams",
            ResourceIdentity::Shares => "Shares",
            ResourceIdentity::Sites => "Sites",
            ResourceIdentity::SitesContentTypes => "SitesContentTypes",
            ResourceIdentity::SitesItems => "SitesItems",
            ResourceIdentity::SitesItemsVersions => "SitesItemsVersions",
            ResourceIdentity::SitesLists => "SitesLists",
            ResourceIdentity::Solutions => "Solutions",
            ResourceIdentity::StaffMembers => "StaffMembers",
            ResourceIdentity::SubscribedSkus => "SubscribedSkus",
            ResourceIdentity::Subscriptions => "Subscriptions",
            ResourceIdentity::Tabs => "Tabs",
            ResourceIdentity::TargetedManagedAppConfigurations => {
                "TargetedManagedAppConfigurations"
            }
            ResourceIdentity::Tasks => "Tasks",
            ResourceIdentity::Teams => "Teams",
            ResourceIdentity::TeamsMembers => "TeamsMembers",
            ResourceIdentity::TeamsPrimaryChannelTabs => "TeamsPrimaryChannelTabs",
            ResourceIdentity::TeamsTags => "TeamsTags",
            ResourceIdentity::TeamsTemplates => "TeamsTemplates",
            ResourceIdentity::Teamwork => "Teamwork",
            ResourceIdentity::TermStore => "TermStore",
            ResourceIdentity::TermStoreGroups => "TermStoreGroups",
            ResourceIdentity::TermStoreSets => "TermStoreSets",
            ResourceIdentity::TermStoreSetsChildren => "TermStoreSetsChildren",
            ResourceIdentity::TermStoreSetsParentGroup => "TermStoreSetsParentGroup",
            ResourceIdentity::TermStoreSetsRelations => "TermStoreSetsRelations",
            ResourceIdentity::TermStoreSetsTerms => "TermStoreSetsTerms",
            ResourceIdentity::TermStores => "TermStores",
            ResourceIdentity::TermsAndConditions => "TermsAndConditions",
            ResourceIdentity::Threads => "Threads",
            ResourceIdentity::ThreadsPosts => "ThreadsPosts",
            ResourceIdentity::Todo => "Todo",
            ResourceIdentity::TodoLists => "TodoLists",
            ResourceIdentity::TodoListsTasks => "TodoListsTasks",
            ResourceIdentity::TransitiveMemberOf => "TransitiveMemberOf",
            ResourceIdentity::TransitiveMembers => "TransitiveMembers",
            ResourceIdentity::TroubleshootingEvents => "TroubleshootingEvents",
            ResourceIdentity::Users => "Users",
            ResourceIdentity::UsersAttachments => "UsersAttachments",
            ResourceIdentity::UsersManagedDevices => "UsersManagedDevices",
            ResourceIdentity::UsersMessages => "UsersMessages",
            ResourceIdentity::VirtualEvents => "VirtualEvents",
            ResourceIdentity::VirtualEventsEvents => "VirtualEventsEvents",
            ResourceIdentity::VirtualEventsSessions => "VirtualEventsSessions",
            ResourceIdentity::VirtualEventsWebinars => "VirtualEventsWebinars",
            ResourceIdentity::VppTokens => "VppTokens",
            ResourceIdentity::WindowsAutopilotDeviceIdentities => {
                "WindowsAutopilotDeviceIdentities"
            }
            ResourceIdentity::WindowsInformationProtectionPolicies => {
                "WindowsInformationProtectionPolicies"
            }
            ResourceIdentity::Workbook => "Workbook",
            ResourceIdentity::WorkbookFunctions => "WorkbookFunctions",
            ResourceIdentity::WorkbookTables => "WorkbookTables",
            ResourceIdentity::WorkbookTablesColumns => "WorkbookTablesColumns",
            ResourceIdentity::WorkbookTablesRows => "WorkbookTablesRows",
            ResourceIdentity::Worksheets => "Worksheets",
            ResourceIdentity::WorksheetsCharts => "WorksheetsCharts",
            ResourceIdentity::WorksheetsChartsAxes => "WorksheetsChartsAxes",
            ResourceIdentity::WorksheetsChartsAxesCategoryAxis => {
                "WorksheetsChartsAxesCategoryAxis"
            }
            ResourceIdentity::WorksheetsChartsAxesSeriesAxis => "WorksheetsChartsAxesSeriesAxis",
            ResourceIdentity::WorksheetsChartsAxesValueAxis => "WorksheetsChartsAxesValueAxis",
            ResourceIdentity::WorksheetsChartsDataLabels => "WorksheetsChartsDataLabels",
            ResourceIdentity::WorksheetsChartsFormat => "WorksheetsChartsFormat",
            ResourceIdentity::WorksheetsChartsLegend => "WorksheetsChartsLegend",
            ResourceIdentity::WorksheetsChartsSeries => "WorksheetsChartsSeries",
            ResourceIdentity::WorksheetsChartsTitle => "WorksheetsChartsTitle",
        }
    }
}

impl FromStr for ResourceIdentity {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AccessPackageAssignmentApprovals" | "accessPackageAssignmentApprovals" => {
                Ok(ResourceIdentity::AccessPackageAssignmentApprovals)
            }
            "AccessPackages" | "accessPackages" => Ok(ResourceIdentity::AccessPackages),
            "AccessReviews" | "accessReviews" => Ok(ResourceIdentity::AccessReviews),
            "AccessReviewsDefinitions" | "accessReviewsDefinitions" => {
                Ok(ResourceIdentity::AccessReviewsDefinitions)
            }
            "AccessReviewsDefinitionsInstances" | "accessReviewsDefinitionsInstances" => {
                Ok(ResourceIdentity::AccessReviewsDefinitionsInstances)
            }
            "AccessReviewsDefinitionsInstancesStages"
            | "accessReviewsDefinitionsInstancesStages" => {
                Ok(ResourceIdentity::AccessReviewsDefinitionsInstancesStages)
            }
            "Activities" | "activities" => Ok(ResourceIdentity::Activities),
            "Admin" | "admin" => Ok(ResourceIdentity::Admin),
            "AdministrativeUnits" | "administrativeUnits" => {
                Ok(ResourceIdentity::AdministrativeUnits)
            }
            "AgreementAcceptances" | "agreementAcceptances" => {
                Ok(ResourceIdentity::AgreementAcceptances)
            }
            "Agreements" | "agreements" => Ok(ResourceIdentity::Agreements),
            "AllChannels" | "allChannels" => Ok(ResourceIdentity::AllChannels),
            "AndroidManagedAppProtections" | "androidManagedAppProtections" => {
                Ok(ResourceIdentity::AndroidManagedAppProtections)
            }
            "AppCatalogs" | "appCatalogs" => Ok(ResourceIdentity::AppCatalogs),
            "AppConsent" | "appConsent" => Ok(ResourceIdentity::AppConsent),
            "Application" | "application" => Ok(ResourceIdentity::Application),
            "Applications" | "applications" => Ok(ResourceIdentity::Applications),
            "ApplicationTemplates" | "applicationTemplates" => {
                Ok(ResourceIdentity::ApplicationTemplates)
            }
            "Appointments" | "appointments" => Ok(ResourceIdentity::Appointments),
            "AppRoleAssignments" | "appRoleAssignments" => Ok(ResourceIdentity::AppRoleAssignments),
            "AssignmentPolicies" | "assignmentPolicies" => Ok(ResourceIdentity::AssignmentPolicies),
            "AssignmentRequests" | "assignmentRequests" => Ok(ResourceIdentity::AssignmentRequests),
            "Assignments" | "assignments" => Ok(ResourceIdentity::Assignments),
            "AuditLogs" | "auditLogs" => Ok(ResourceIdentity::AuditLogs),
            "Authentication" | "authentication" => Ok(ResourceIdentity::Authentication),
            "AuthenticationMethodConfigurations" | "authenticationMethodConfigurations" => {
                Ok(ResourceIdentity::AuthenticationMethodConfigurations)
            }
            "AuthenticationMethodsPolicy" | "authenticationMethodsPolicy" => {
                Ok(ResourceIdentity::AuthenticationMethodsPolicy)
            }
            "Batch" | "batch" => Ok(ResourceIdentity::Batch),
            "BookingBusinesses" | "bookingBusinesses" => Ok(ResourceIdentity::BookingBusinesses),
            "Branding" | "branding" => Ok(ResourceIdentity::Branding),
            "Buckets" | "buckets" => Ok(ResourceIdentity::Buckets),
            "CalendarGroups" | "calendarGroups" => Ok(ResourceIdentity::CalendarGroups),
            "Calendars" | "calendars" => Ok(ResourceIdentity::Calendars),
            "CalendarView" | "calendarView" => Ok(ResourceIdentity::CalendarView),
            "CallRecords" | "callRecords" => Ok(ResourceIdentity::CallRecords),
            "CallRecordsSessions" | "callRecordsSessions" => {
                Ok(ResourceIdentity::CallRecordsSessions)
            }
            "Calls" | "calls" => Ok(ResourceIdentity::Calls),
            "CertificateBasedAuthConfiguration" | "certificateBasedAuthConfiguration" => {
                Ok(ResourceIdentity::CertificateBasedAuthConfiguration)
            }
            "Channels" | "channels" => Ok(ResourceIdentity::Channels),
            "Chats" | "chats" => Ok(ResourceIdentity::Chats),
            "ChatsAndChannelsMessages" | "chatsAndChannelsMessages" => {
                Ok(ResourceIdentity::ChatsAndChannelsMessages)
            }
            "ChatsMessages" | "chatsMessages" => Ok(ResourceIdentity::ChatsMessages),
            "ChatsMessagesReplies" | "chatsMessagesReplies" => {
                Ok(ResourceIdentity::ChatsMessagesReplies)
            }
            "ChildFolders" | "childFolders" => Ok(ResourceIdentity::ChildFolders),
            "Communications" | "communications" => Ok(ResourceIdentity::Communications),
            "Compliance" | "compliance" => Ok(ResourceIdentity::Compliance),
            "ConnectedOrganizations" | "connectedOrganizations" => {
                Ok(ResourceIdentity::ConnectedOrganizations)
            }
            "ConnectedOrganizationsExternalSponsors" | "connectedOrganizationsExternalSponsors" => {
                Ok(ResourceIdentity::ConnectedOrganizationsExternalSponsors)
            }
            "ConnectedOrganizationsInternalSponsors" | "connectedOrganizationsInternalSponsors" => {
                Ok(ResourceIdentity::ConnectedOrganizationsInternalSponsors)
            }
            "Connections" | "connections" => Ok(ResourceIdentity::Connections),
            "ContactFolders" | "contactFolders" => Ok(ResourceIdentity::ContactFolders),
            "Contacts" | "contacts" => Ok(ResourceIdentity::Contacts),
            "ContentTypes" | "contentTypes" => Ok(ResourceIdentity::ContentTypes),
            "Contracts" | "contracts" => Ok(ResourceIdentity::Contracts),
            "Conversations" | "conversations" => Ok(ResourceIdentity::Conversations),
            "CreatedByUser" | "createdByUser" => Ok(ResourceIdentity::CreatedByUser),
            "CreatedObjects" | "createdObjects" => Ok(ResourceIdentity::CreatedObjects),
            "Custom" | "custom" => Ok(ResourceIdentity::Custom),
            "Customers" | "customers" => Ok(ResourceIdentity::Customers),
            "CustomQuestions" | "customQuestions" => Ok(ResourceIdentity::CustomQuestions),
            "DataPolicyOperations" | "dataPolicyOperations" => {
                Ok(ResourceIdentity::DataPolicyOperations)
            }
            "DefaultCalendar" | "defaultCalendar" => Ok(ResourceIdentity::DefaultCalendar),
            "DefaultManagedAppProtections" | "defaultManagedAppProtections" => {
                Ok(ResourceIdentity::DefaultManagedAppProtections)
            }
            "DeletedItems" | "deletedItems" => Ok(ResourceIdentity::DeletedItems),
            "DeletedTeams" | "deletedTeams" => Ok(ResourceIdentity::DeletedTeams),
            "DeviceAppManagement" | "deviceAppManagement" => {
                Ok(ResourceIdentity::DeviceAppManagement)
            }
            "DeviceCompliancePolicies" | "deviceCompliancePolicies" => {
                Ok(ResourceIdentity::DeviceCompliancePolicies)
            }
            "DeviceCompliancePolicySettingStateSummaries"
            | "deviceCompliancePolicySettingStateSummaries" => {
                Ok(ResourceIdentity::DeviceCompliancePolicySettingStateSummaries)
            }
            "DeviceConfigurations" | "deviceConfigurations" => {
                Ok(ResourceIdentity::DeviceConfigurations)
            }
            "DeviceEnrollmentConfigurations" | "deviceEnrollmentConfigurations" => {
                Ok(ResourceIdentity::DeviceEnrollmentConfigurations)
            }
            "DeviceManagement" | "deviceManagement" => Ok(ResourceIdentity::DeviceManagement),
            "DeviceManagementManagedDevices" | "deviceManagementManagedDevices" => {
                Ok(ResourceIdentity::DeviceManagementManagedDevices)
            }
            "DeviceManagementReports" | "deviceManagementReports" => {
                Ok(ResourceIdentity::DeviceManagementReports)
            }
            "Devices" | "devices" => Ok(ResourceIdentity::Devices),
            "DevicesRegisteredOwners" | "devicesRegisteredOwners" => {
                Ok(ResourceIdentity::DevicesRegisteredOwners)
            }
            "DevicesRegisteredUsers" | "devicesRegisteredUsers" => {
                Ok(ResourceIdentity::DevicesRegisteredUsers)
            }
            "Directory" | "directory" => Ok(ResourceIdentity::Directory),
            "DirectoryMembers" | "directoryMembers" => Ok(ResourceIdentity::DirectoryMembers),
            "DirectoryObjects" | "directoryObjects" => Ok(ResourceIdentity::DirectoryObjects),
            "DirectoryRoles" | "directoryRoles" => Ok(ResourceIdentity::DirectoryRoles),
            "DirectoryRoleTemplates" | "directoryRoleTemplates" => {
                Ok(ResourceIdentity::DirectoryRoleTemplates)
            }
            "DirectReports" | "directReports" => Ok(ResourceIdentity::DirectReports),
            "DomainDnsRecords" | "domainDnsRecords" => Ok(ResourceIdentity::DomainDnsRecords),
            "Domains" | "domains" => Ok(ResourceIdentity::Domains),
            "Drive" | "drive" => Ok(ResourceIdentity::Drive),
            "Drives" | "drives" => Ok(ResourceIdentity::Drives),
            "DrivesItems" | "drivesItems" => Ok(ResourceIdentity::DrivesItems),
            "DrivesList" | "drivesList" => Ok(ResourceIdentity::DrivesList),
            "DrivesListContentTypes" | "drivesListContentTypes" => {
                Ok(ResourceIdentity::DrivesListContentTypes)
            }
            "Education" | "education" => Ok(ResourceIdentity::Education),
            "EducationAssignments" | "educationAssignments" => {
                Ok(ResourceIdentity::EducationAssignments)
            }
            "EducationAssignmentsSubmissions" | "educationAssignmentsSubmissions" => {
                Ok(ResourceIdentity::EducationAssignmentsSubmissions)
            }
            "EducationClasses" | "educationClasses" => Ok(ResourceIdentity::EducationClasses),
            "EducationMe" | "educationMe" => Ok(ResourceIdentity::EducationMe),
            "EducationSchools" | "educationSchools" => Ok(ResourceIdentity::EducationSchools),
            "EducationUsers" | "educationUsers" => Ok(ResourceIdentity::EducationUsers),
            "EntitlementManagement" | "entitlementManagement" => {
                Ok(ResourceIdentity::EntitlementManagement)
            }
            "EntitlementManagementAssignments" | "entitlementManagementAssignments" => {
                Ok(ResourceIdentity::EntitlementManagementAssignments)
            }
            "EntitlementManagementCatalogs" | "entitlementManagementCatalogs" => {
                Ok(ResourceIdentity::EntitlementManagementCatalogs)
            }
            "Events" | "events" => Ok(ResourceIdentity::Events),
            "EventsInstances" | "eventsInstances" => Ok(ResourceIdentity::EventsInstances),
            "ExtendedProperties" | "extendedProperties" => Ok(ResourceIdentity::ExtendedProperties),
            "Extensions" | "extensions" => Ok(ResourceIdentity::Extensions),
            "External" | "external" => Ok(ResourceIdentity::External),
            "FollowedSites" | "followedSites" => Ok(ResourceIdentity::FollowedSites),
            "GroupLifecyclePolicies" | "groupLifecyclePolicies" => {
                Ok(ResourceIdentity::GroupLifecyclePolicies)
            }
            "GroupSettings" | "groupSettings" => Ok(ResourceIdentity::GroupSettings),
            "GroupSettingTemplates" | "groupSettingTemplates" => {
                Ok(ResourceIdentity::GroupSettingTemplates)
            }
            "Groups" | "groups" => Ok(ResourceIdentity::Groups),
            "GroupsOwners" | "groupsOwners" => Ok(ResourceIdentity::GroupsOwners),
            "GroupsTeam" | "groupsTeam" => Ok(ResourceIdentity::GroupsTeam),
            "HistoryItems" | "historyItems" => Ok(ResourceIdentity::HistoryItems),
            "Identity" | "identity" => Ok(ResourceIdentity::Identity),
            "IdentityGovernance" | "identityGovernance" => Ok(ResourceIdentity::IdentityGovernance),
            "IdentityProtection" | "identityProtection" => Ok(ResourceIdentity::IdentityProtection),
            "IdentityProviders" | "identityProviders" => Ok(ResourceIdentity::IdentityProviders),
            "IncomingChannels" | "incomingChannels" => Ok(ResourceIdentity::IncomingChannels),
            "InferenceClassification" | "inferenceClassification" => {
                Ok(ResourceIdentity::InferenceClassification)
            }
            "InformationProtection" | "informationProtection" => {
                Ok(ResourceIdentity::InformationProtection)
            }
            "Insights" | "insights" => Ok(ResourceIdentity::Insights),
            "Invitations" | "invitations" => Ok(ResourceIdentity::Invitations),
            "IosManagedAppProtections" | "iosManagedAppProtections" => {
                Ok(ResourceIdentity::IosManagedAppProtections)
            }
            "JoinedTeams" | "joinedTeams" => Ok(ResourceIdentity::JoinedTeams),
            "LastModifiedByUser" | "lastModifiedByUser" => Ok(ResourceIdentity::LastModifiedByUser),
            "LicenseDetails" | "licenseDetails" => Ok(ResourceIdentity::LicenseDetails),
            "List" | "list" => Ok(ResourceIdentity::List),
            "Localizations" | "localizations" => Ok(ResourceIdentity::Localizations),
            "MailboxSettings" | "mailboxSettings" => Ok(ResourceIdentity::MailboxSettings),
            "MailFolders" | "mailFolders" => Ok(ResourceIdentity::MailFolders),
            "ManagedAppPolicies" | "managedAppPolicies" => Ok(ResourceIdentity::ManagedAppPolicies),
            "ManagedAppRegistrations" | "managedAppRegistrations" => {
                Ok(ResourceIdentity::ManagedAppRegistrations)
            }
            "ManagedAppRegistrationsAppliedPolicies" | "managedAppRegistrationsAppliedPolicies" => {
                Ok(ResourceIdentity::ManagedAppRegistrationsAppliedPolicies)
            }
            "ManagedAppRegistrationsIntendedPolicies"
            | "managedAppRegistrationsIntendedPolicies" => {
                Ok(ResourceIdentity::ManagedAppRegistrationsIntendedPolicies)
            }
            "ManagedAppStatuses" | "managedAppStatuses" => Ok(ResourceIdentity::ManagedAppStatuses),
            "ManagedEBooks" | "managedEBooks" => Ok(ResourceIdentity::ManagedEBooks),
            "ManagedEBooksDeviceStates" | "managedEBooksDeviceStates" => {
                Ok(ResourceIdentity::ManagedEBooksDeviceStates)
            }
            "ManagedEBooksUserStateSummary" | "managedEBooksUserStateSummary" => {
                Ok(ResourceIdentity::ManagedEBooksUserStateSummary)
            }
            "MdmWindowsInformationProtectionPolicies"
            | "mdmWindowsInformationProtectionPolicies" => {
                Ok(ResourceIdentity::MdmWindowsInformationProtectionPolicies)
            }
            "Me" | "me" => Ok(ResourceIdentity::Me),
            "MemberOf" | "memberOf" => Ok(ResourceIdentity::MemberOf),
            "MembersWithLicenseErrors" | "membersWithLicenseErrors" => {
                Ok(ResourceIdentity::MembersWithLicenseErrors)
            }
            "MobileAppCategories" | "mobileAppCategories" => {
                Ok(ResourceIdentity::MobileAppCategories)
            }
            "MobileAppConfigurations" | "mobileAppConfigurations" => {
                Ok(ResourceIdentity::MobileAppConfigurations)
            }
            "MobileApps" | "mobileApps" => Ok(ResourceIdentity::MobileApps),
            "Oauth2PermissionGrants" | "oauth2PermissionGrants" => {
                Ok(ResourceIdentity::Oauth2PermissionGrants)
            }
            "Onenote" | "onenote" => Ok(ResourceIdentity::Onenote),
            "OnenoteNotebooks" | "onenoteNotebooks" => Ok(ResourceIdentity::OnenoteNotebooks),
            "OnenotePages" | "onenotePages" => Ok(ResourceIdentity::OnenotePages),
            "OnenoteSectionGroups" | "onenoteSectionGroups" => {
                Ok(ResourceIdentity::OnenoteSectionGroups)
            }
            "OnenoteSections" | "onenoteSections" => Ok(ResourceIdentity::OnenoteSections),
            "OnlineMeetings" | "onlineMeetings" => Ok(ResourceIdentity::OnlineMeetings),
            "Organization" | "organization" => Ok(ResourceIdentity::Organization),
            "OrgContacts" | "orgContacts" => Ok(ResourceIdentity::OrgContacts),
            "Outlook" | "outlook" => Ok(ResourceIdentity::Outlook),
            "OwnedDevices" | "ownedDevices" => Ok(ResourceIdentity::OwnedDevices),
            "OwnedObjects" | "ownedObjects" => Ok(ResourceIdentity::OwnedObjects),
            "ParentNotebook" | "parentNotebook" => Ok(ResourceIdentity::ParentNotebook),
            "ParentSection" | "parentSection" => Ok(ResourceIdentity::ParentSection),
            "ParentSectionGroup" | "parentSectionGroup" => Ok(ResourceIdentity::ParentSectionGroup),
            "People" | "people" => Ok(ResourceIdentity::People),
            "PermissionGrants" | "permissionGrants" => Ok(ResourceIdentity::PermissionGrants),
            "Photos" | "photos" => Ok(ResourceIdentity::Photos),
            "Places" | "places" => Ok(ResourceIdentity::Places),
            "Planner" | "planner" => Ok(ResourceIdentity::Planner),
            "PlannerTasks" | "plannerTasks" => Ok(ResourceIdentity::PlannerTasks),
            "Plans" | "plans" => Ok(ResourceIdentity::Plans),
            "Policies" | "policies" => Ok(ResourceIdentity::Policies),
            "Presence" | "presence" => Ok(ResourceIdentity::Presence),
            "PrimaryChannel" | "primaryChannel" => Ok(ResourceIdentity::PrimaryChannel),
            "Print" | "print" => Ok(ResourceIdentity::Print),
            "Privacy" | "privacy" => Ok(ResourceIdentity::Privacy),
            "RegisteredDevices" | "registeredDevices" => Ok(ResourceIdentity::RegisteredDevices),
            "Reports" | "reports" => Ok(ResourceIdentity::Reports),
            "RoleDefinitions" | "roleDefinitions" => Ok(ResourceIdentity::RoleDefinitions),
            "RoleManagement" | "roleManagement" => Ok(ResourceIdentity::RoleManagement),
            "Schedule" | "schedule" => Ok(ResourceIdentity::Schedule),
            "SchemaExtensions" | "schemaExtensions" => Ok(ResourceIdentity::SchemaExtensions),
            "ScopedRoleMemberOf" | "scopedRoleMemberOf" => Ok(ResourceIdentity::ScopedRoleMemberOf),
            "ScopedRoleMemberships" | "scopedRoleMemberships" => {
                Ok(ResourceIdentity::ScopedRoleMemberships)
            }
            "Search" | "search" => Ok(ResourceIdentity::Search),
            "Security" | "security" => Ok(ResourceIdentity::Security),
            "ServicePrincipals" | "servicePrincipals" => Ok(ResourceIdentity::ServicePrincipals),
            "ServicePrincipalsOwners" | "servicePrincipalsOwners" => {
                Ok(ResourceIdentity::ServicePrincipalsOwners)
            }
            "Services" | "services" => Ok(ResourceIdentity::Services),
            "Settings" | "settings" => Ok(ResourceIdentity::Settings),
            "SharedWithTeams" | "sharedWithTeams" => Ok(ResourceIdentity::SharedWithTeams),
            "Shares" | "shares" => Ok(ResourceIdentity::Shares),
            "Sites" | "sites" => Ok(ResourceIdentity::Sites),
            "SitesContentTypes" | "sitesContentTypes" => Ok(ResourceIdentity::SitesContentTypes),
            "SitesItems" | "sitesItems" => Ok(ResourceIdentity::SitesItems),
            "SitesItemsVersions" | "sitesItemsVersions" => Ok(ResourceIdentity::SitesItemsVersions),
            "SitesLists" | "sitesLists" => Ok(ResourceIdentity::SitesLists),
            "Solutions" | "solutions" => Ok(ResourceIdentity::Solutions),
            "StaffMembers" | "staffMembers" => Ok(ResourceIdentity::StaffMembers),
            "SubscribedSkus" | "subscribedSkus" => Ok(ResourceIdentity::SubscribedSkus),
            "Subscriptions" | "subscriptions" => Ok(ResourceIdentity::Subscriptions),
            "Tabs" | "tabs" => Ok(ResourceIdentity::Tabs),
            "TargetedManagedAppConfigurations" | "targetedManagedAppConfigurations" => {
                Ok(ResourceIdentity::TargetedManagedAppConfigurations)
            }
            "Tasks" | "tasks" => Ok(ResourceIdentity::Tasks),
            "Teams" | "teams" => Ok(ResourceIdentity::Teams),
            "TeamsMembers" | "teamsMembers" => Ok(ResourceIdentity::TeamsMembers),
            "TeamsPrimaryChannelTabs" | "teamsPrimaryChannelTabs" => {
                Ok(ResourceIdentity::TeamsPrimaryChannelTabs)
            }
            "TeamsTags" | "teamsTags" => Ok(ResourceIdentity::TeamsTags),
            "TeamsTemplates" | "teamsTemplates" => Ok(ResourceIdentity::TeamsTemplates),
            "Teamwork" | "teamwork" => Ok(ResourceIdentity::Teamwork),
            "TermStore" | "termStore" => Ok(ResourceIdentity::TermStore),
            "TermStoreGroups" | "termStoreGroups" => Ok(ResourceIdentity::TermStoreGroups),
            "TermStoreSets" | "termStoreSets" => Ok(ResourceIdentity::TermStoreSets),
            "TermStoreSetsChildren" | "termStoreSetsChildren" => {
                Ok(ResourceIdentity::TermStoreSetsChildren)
            }
            "TermStoreSetsParentGroup" | "termStoreSetsParentGroup" => {
                Ok(ResourceIdentity::TermStoreSetsParentGroup)
            }
            "TermStoreSetsRelations" | "termStoreSetsRelations" => {
                Ok(ResourceIdentity::TermStoreSetsRelations)
            }
            "TermStoreSetsTerms" | "termStoreSetsTerms" => Ok(ResourceIdentity::TermStoreSetsTerms),
            "TermStores" | "termStores" => Ok(ResourceIdentity::TermStores),
            "TermsAndConditions" | "termsAndConditions" => Ok(ResourceIdentity::TermsAndConditions),
            "Threads" | "threads" => Ok(ResourceIdentity::Threads),
            "ThreadsPosts" | "threadsPosts" => Ok(ResourceIdentity::ThreadsPosts),
            "Todo" | "todo" => Ok(ResourceIdentity::Todo),
            "TodoLists" | "todoLists" => Ok(ResourceIdentity::TodoLists),
            "TodoListsTasks" | "todoListsTasks" => Ok(ResourceIdentity::TodoListsTasks),
            "TransitiveMemberOf" | "transitiveMemberOf" => Ok(ResourceIdentity::TransitiveMemberOf),
            "TransitiveMembers" | "transitiveMembers" => Ok(ResourceIdentity::TransitiveMembers),
            "TroubleshootingEvents" | "troubleshootingEvents" => {
                Ok(ResourceIdentity::TroubleshootingEvents)
            }
            "Users" | "users" => Ok(ResourceIdentity::Users),
            "UsersAttachments" | "usersAttachments" => Ok(ResourceIdentity::UsersAttachments),
            "UsersManagedDevices" | "usersManagedDevices" => {
                Ok(ResourceIdentity::UsersManagedDevices)
            }
            "UsersMessages" | "usersMessages" => Ok(ResourceIdentity::UsersMessages),
            "VirtualEvents" | "virtualEvents" => Ok(ResourceIdentity::VirtualEvents),
            "VirtualEventsEvents" | "virtualEventsEvents" => {
                Ok(ResourceIdentity::VirtualEventsEvents)
            }
            "VirtualEventsSessions" | "virtualEventsSessions" => {
                Ok(ResourceIdentity::VirtualEventsSessions)
            }
            "VirtualEventsWebinars" | "virtualEventsWebinars" => {
                Ok(ResourceIdentity::VirtualEventsWebinars)
            }
            "VppTokens" | "vppTokens" => Ok(ResourceIdentity::VppTokens),
            "WindowsAutopilotDeviceIdentities" | "windowsAutopilotDeviceIdentities" => {
                Ok(ResourceIdentity::WindowsAutopilotDeviceIdentities)
            }
            "WindowsInformationProtectionPolicies" | "windowsInformationProtectionPolicies" => {
                Ok(ResourceIdentity::WindowsInformationProtectionPolicies)
            }
            "Workbook" | "workbook" => Ok(ResourceIdentity::Workbook),
            "WorkbookFunctions" | "workbookFunctions" => Ok(ResourceIdentity::WorkbookFunctions),
            "WorkbookTables" | "workbookTables" => Ok(ResourceIdentity::WorkbookTables),
            "WorkbookTablesColumns" | "workbookTablesColumns" => {
                Ok(ResourceIdentity::WorkbookTablesColumns)
            }
            "WorkbookTablesRows" | "workbookTablesRows" => Ok(ResourceIdentity::WorkbookTablesRows),
            "Worksheets" | "worksheets" => Ok(ResourceIdentity::Worksheets),
            "WorksheetsCharts" | "worksheetsCharts" => Ok(ResourceIdentity::WorksheetsCharts),
            "WorksheetsChartsAxes" | "worksheetsChartsAxes" => {
                Ok(ResourceIdentity::WorksheetsChartsAxes)
            }
            "WorksheetsChartsAxesCategoryAxis" | "worksheetsChartsAxesCategoryAxis" => {
                Ok(ResourceIdentity::WorksheetsChartsAxesCategoryAxis)
            }
            "WorksheetsChartsAxesSeriesAxis" | "worksheetsChartsAxesSeriesAxis" => {
                Ok(ResourceIdentity::WorksheetsChartsAxesSeriesAxis)
            }
            "WorksheetsChartsAxesValueAxis" | "worksheetsChartsAxesValueAxis" => {
                Ok(ResourceIdentity::WorksheetsChartsAxesValueAxis)
            }
            "WorksheetsChartsDataLabels" | "worksheetsChartsDataLabels" => {
                Ok(ResourceIdentity::WorksheetsChartsDataLabels)
            }
            "WorksheetsChartsFormat" | "worksheetsChartsFormat" => {
                Ok(ResourceIdentity::WorksheetsChartsFormat)
            }
            "WorksheetsChartsLegend" | "worksheetsChartsLegend" => {
                Ok(ResourceIdentity::WorksheetsChartsLegend)
            }
            "WorksheetsChartsSeries" | "worksheetsChartsSeries" => {
                Ok(ResourceIdentity::WorksheetsChartsSeries)
            }
            "WorksheetsChartsTitle" | "worksheetsChartsTitle" => {
                Ok(ResourceIdentity::WorksheetsChartsTitle)
            }
            _ => Err(format!("Unknown ResourceIdentity: {}", s)),
        }
    }
}

impl AsRef<str> for TopLevelResource {
    fn as_ref(&self) -> &str {
        match self {
            TopLevelResource::Admin => "Admin",
            TopLevelResource::AgreementAcceptances => "AgreementAcceptances",
            TopLevelResource::Agreements => "Agreements",
            TopLevelResource::AppCatalogs => "AppCatalogs",
            TopLevelResource::ApplicationTemplates => "ApplicationTemplates",
            TopLevelResource::Applications => "Applications",
            TopLevelResource::AuditLogs => "AuditLogs",
            TopLevelResource::AuthenticationMethodConfigurations => {
                "AuthenticationMethodConfigurations"
            }
            TopLevelResource::AuthenticationMethodsPolicy => "AuthenticationMethodsPolicy",
            TopLevelResource::Branding => "Branding",
            TopLevelResource::CertificateBasedAuthConfiguration => {
                "CertificateBasedAuthConfiguration"
            }
            TopLevelResource::Chats => "Chats",
            TopLevelResource::Communications => "Communications",
            TopLevelResource::Compliance => "Compliance",
            TopLevelResource::Connections => "Connections",
            TopLevelResource::Contacts => "Contacts",
            TopLevelResource::Contracts => "Contracts",
            TopLevelResource::DataPolicyOperations => "DataPolicyOperations",
            TopLevelResource::DeviceAppManagement => "DeviceAppManagement",
            TopLevelResource::DeviceManagement => "DeviceManagement",
            TopLevelResource::Devices => "Devices",
            TopLevelResource::Directory => "Directory",
            TopLevelResource::DirectoryObjects => "DirectoryObjects",
            TopLevelResource::DirectoryRoleTemplates => "DirectoryRoleTemplates",
            TopLevelResource::DirectoryRoles => "DirectoryRoles",
            TopLevelResource::DomainDnsRecords => "DomainDnsRecords",
            TopLevelResource::Domains => "Domains",
            TopLevelResource::Drive => "Drive",
            TopLevelResource::Drives => "Drives",
            TopLevelResource::Education => "Education",
            TopLevelResource::External => "External",
            TopLevelResource::GroupLifecyclePolicies => "GroupLifecyclePolicies",
            TopLevelResource::GroupSettingTemplates => "GroupSettingTemplates",
            TopLevelResource::GroupSettings => "GroupSettings",
            TopLevelResource::Groups => "Groups",
            TopLevelResource::Identity => "Identity",
            TopLevelResource::IdentityGovernance => "IdentityGovernance",
            TopLevelResource::IdentityProtection => "IdentityProtection",
            TopLevelResource::IdentityProviders => "IdentityProviders",
            TopLevelResource::InformationProtection => "InformationProtection",
            TopLevelResource::Invitations => "Invitations",
            TopLevelResource::Localizations => "Localizations",
            TopLevelResource::Me => "Me",
            TopLevelResource::Oauth2PermissionGrants => "Oauth2PermissionGrants",
            TopLevelResource::Organization => "Organization",
            TopLevelResource::PermissionGrants => "PermissionGrants",
            TopLevelResource::Places => "Places",
            TopLevelResource::Planner => "Planner",
            TopLevelResource::Policies => "Policies",
            TopLevelResource::Print => "Print",
            TopLevelResource::Privacy => "Privacy",
            TopLevelResource::Reports => "Reports",
            TopLevelResource::RoleManagement => "RoleManagement",
            TopLevelResource::SchemaExtensions => "SchemaExtensions",
            TopLevelResource::ScopedRoleMemberships => "ScopedRoleMemberships",
            TopLevelResource::Search => "Search",
            TopLevelResource::Security => "Security",
            TopLevelResource::ServicePrincipals => "ServicePrincipals",
            TopLevelResource::Shares => "Shares",
            TopLevelResource::Sites => "Sites",
            TopLevelResource::Solutions => "Solutions",
            TopLevelResource::SubscribedSkus => "SubscribedSkus",
            TopLevelResource::Subscriptions => "Subscriptions",
            TopLevelResource::Teams => "Teams",
            TopLevelResource::TeamsTemplates => "TeamsTemplates",
            TopLevelResource::Teamwork => "Teamwork",
            TopLevelResource::Users => "Users",
        }
    }
}

impl FromStr for TopLevelResource {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Admin" | "admin" => Ok(TopLevelResource::Admin),
            "AgreementAcceptances" | "agreementAcceptances" => {
                Ok(TopLevelResource::AgreementAcceptances)
            }
            "Agreements" | "agreements" => Ok(TopLevelResource::Agreements),
            "AppCatalogs" | "appCatalogs" => Ok(TopLevelResource::AppCatalogs),
            "ApplicationTemplates" | "applicationTemplates" => {
                Ok(TopLevelResource::ApplicationTemplates)
            }
            "Applications" | "applications" => Ok(TopLevelResource::Applications),
            "AuditLogs" | "auditLogs" => Ok(TopLevelResource::AuditLogs),
            "AuthenticationMethodConfigurations" | "authenticationMethodConfigurations" => {
                Ok(TopLevelResource::AuthenticationMethodConfigurations)
            }
            "AuthenticationMethodsPolicy" | "authenticationMethodsPolicy" => {
                Ok(TopLevelResource::AuthenticationMethodsPolicy)
            }
            "Branding" | "branding" => Ok(TopLevelResource::Branding),
            "CertificateBasedAuthConfiguration" | "certificateBasedAuthConfiguration" => {
                Ok(TopLevelResource::CertificateBasedAuthConfiguration)
            }
            "Chats" | "chats" => Ok(TopLevelResource::Chats),
            "Communications" | "communications" => Ok(TopLevelResource::Communications),
            "Compliance" | "compliance" => Ok(TopLevelResource::Compliance),
            "Connections" | "connections" => Ok(TopLevelResource::Connections),
            "Contacts" | "contacts" => Ok(TopLevelResource::Contacts),
            "Contracts" | "contracts" => Ok(TopLevelResource::Contracts),
            "DataPolicyOperations" | "dataPolicyOperations" => {
                Ok(TopLevelResource::DataPolicyOperations)
            }
            "DeviceAppManagement" | "deviceAppManagement" => {
                Ok(TopLevelResource::DeviceAppManagement)
            }
            "DeviceManagement" | "deviceManagement" => Ok(TopLevelResource::DeviceManagement),
            "Devices" | "devices" => Ok(TopLevelResource::Devices),
            "Directory" | "directory" => Ok(TopLevelResource::Directory),
            "DirectoryObjects" | "directoryObjects" => Ok(TopLevelResource::DirectoryObjects),
            "DirectoryRoleTemplates" | "directoryRoleTemplates" => {
                Ok(TopLevelResource::DirectoryRoleTemplates)
            }
            "DirectoryRoles" | "directoryRoles" => Ok(TopLevelResource::DirectoryRoles),
            "DomainDnsRecords" | "domainDnsRecords" => Ok(TopLevelResource::DomainDnsRecords),
            "Domains" | "domains" => Ok(TopLevelResource::Domains),
            "Drive" | "drive" => Ok(TopLevelResource::Drive),
            "Drives" | "drives" => Ok(TopLevelResource::Drives),
            "Education" | "education" => Ok(TopLevelResource::Education),
            "External" | "external" => Ok(TopLevelResource::External),
            "GroupLifecyclePolicies" | "groupLifecyclePolicies" => {
                Ok(TopLevelResource::GroupLifecyclePolicies)
            }
            "GroupSettingTemplates" | "groupSettingTemplates" => {
                Ok(TopLevelResource::GroupSettingTemplates)
            }
            "GroupSettings" | "groupSettings" => Ok(TopLevelResource::GroupSettings),
            "Groups" | "groups" => Ok(TopLevelResource::Groups),
            "Identity" | "identity" => Ok(TopLevelResource::Identity),
            "IdentityGovernance" | "identityGovernance" => Ok(TopLevelResource::IdentityGovernance),
            "IdentityProtection" | "identityProtection" => Ok(TopLevelResource::IdentityProtection),
            "IdentityProviders" | "identityProviders" => Ok(TopLevelResource::IdentityProviders),
            "InformationProtection" | "informationProtection" => {
                Ok(TopLevelResource::InformationProtection)
            }
            "Invitations" | "invitations" => Ok(TopLevelResource::Invitations),
            "Localizations" | "localizations" => Ok(TopLevelResource::Localizations),
            "Me" | "me" => Ok(TopLevelResource::Me),
            "Oauth2PermissionGrants" | "oauth2PermissionGrants" => {
                Ok(TopLevelResource::Oauth2PermissionGrants)
            }
            "Organization" | "organization" => Ok(TopLevelResource::Organization),
            "PermissionGrants" | "permissionGrants" => Ok(TopLevelResource::PermissionGrants),
            "Places" | "places" => Ok(TopLevelResource::Places),
            "Planner" | "planner" => Ok(TopLevelResource::Planner),
            "Policies" | "policies" => Ok(TopLevelResource::Policies),
            "Print" | "print" => Ok(TopLevelResource::Print),
            "Privacy" | "privacy" => Ok(TopLevelResource::Privacy),
            "Reports" | "reports" => Ok(TopLevelResource::Reports),
            "RoleManagement" | "roleManagement" => Ok(TopLevelResource::RoleManagement),
            "SchemaExtensions" | "schemaExtensions" => Ok(TopLevelResource::SchemaExtensions),
            "ScopedRoleMemberships" | "scopedRoleMemberships" => {
                Ok(TopLevelResource::ScopedRoleMemberships)
            }
            "Search" | "search" => Ok(TopLevelResource::Search),
            "Security" | "security" => Ok(TopLevelResource::Security),
            "ServicePrincipals" | "servicePrincipals" => Ok(TopLevelResource::ServicePrincipals),
            "Shares" | "shares" => Ok(TopLevelResource::Shares),
            "Sites" | "sites" => Ok(TopLevelResource::Sites),
            "Solutions" | "solutions" => Ok(TopLevelResource::Solutions),
            "SubscribedSkus" | "subscribedSkus" => Ok(TopLevelResource::SubscribedSkus),
            "Subscriptions" | "subscriptions" => Ok(TopLevelResource::Subscriptions),
            "Teams" | "teams" => Ok(TopLevelResource::Teams),
            "TeamsTemplates" | "teamsTemplates" => Ok(TopLevelResource::TeamsTemplates),
            "Teamwork" | "teamwork" => Ok(TopLevelResource::Teamwork),
            "Users" | "users" => Ok(TopLevelResource::Users),
            _ => Err(format!("Unknown TopLevelResource: {}", s)),
        }
    }
}
