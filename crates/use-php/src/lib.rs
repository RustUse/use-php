#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

#[cfg(feature = "composer-json")]
pub use use_composer_json as composer_json;
#[cfg(feature = "drupal")]
pub use use_drupal as drupal;
#[cfg(feature = "laravel")]
pub use use_laravel as laravel;
#[cfg(feature = "packagist")]
pub use use_packagist as packagist;
#[cfg(feature = "attribute")]
pub use use_php_attribute as attribute;
#[cfg(feature = "autoload")]
pub use use_php_autoload as autoload;
#[cfg(feature = "docblock")]
pub use use_php_docblock as docblock;
#[cfg(feature = "error")]
pub use use_php_error as error;
#[cfg(feature = "extension")]
pub use use_php_extension as extension;
#[cfg(feature = "ini")]
pub use use_php_ini as ini;
#[cfg(feature = "namespace")]
pub use use_php_namespace as namespace;
#[cfg(feature = "symbol")]
pub use use_php_symbol as symbol;
#[cfg(feature = "syntax")]
pub use use_php_syntax as syntax;
#[cfg(feature = "token")]
pub use use_php_token as token;
#[cfg(feature = "type")]
pub use use_php_type as type_;
#[cfg(feature = "version")]
pub use use_php_version as version;
#[cfg(feature = "psr")]
pub use use_psr as psr;
#[cfg(feature = "wordpress")]
pub use use_wordpress as wordpress;
#[cfg(feature = "wordpress-block")]
pub use use_wordpress_block as wordpress_block;
#[cfg(feature = "wordpress-hook")]
pub use use_wordpress_hook as wordpress_hook;

#[cfg(feature = "composer-json")]
pub use use_composer_json::{
    ComposerJson, ComposerPackageName, ComposerRequirement, ComposerStability,
};
#[cfg(feature = "drupal")]
pub use use_drupal::{DrupalConfigObjectName, DrupalModuleName, DrupalPermission, DrupalRouteName};
#[cfg(feature = "laravel")]
pub use use_laravel::{ArtisanCommandName, LaravelConfigKey, LaravelMetadata, LaravelRouteName};
#[cfg(feature = "packagist")]
pub use use_packagist::{PackagistDownloadCount, PackagistPackageName, PackagistStability};
#[cfg(feature = "autoload")]
pub use use_php_autoload::{
    AutoloadConfig, AutoloadPath, AutoloadStrategy, Psr4Mapping, Psr4Prefix,
};
#[cfg(feature = "docblock")]
pub use use_php_docblock::{Docblock, DocblockTag, DocblockTagKind, DocblockTypeString, TagName};
#[cfg(feature = "error")]
pub use use_php_error::{
    DiagnosticMessage, PhpDiagnostic, PhpErrorKind, PhpErrorLevel, PhpSeverity,
};
#[cfg(feature = "extension")]
pub use use_php_extension::{PhpExtensionKind, PhpExtensionName, PhpExtensionRequirement};
#[cfg(feature = "ini")]
pub use use_php_ini::{PhpIniDirective, PhpIniDirectiveName, PhpIniEnvironment, PhpIniValue};
#[cfg(feature = "namespace")]
pub use use_php_namespace::{
    PhpFullyQualifiedName, PhpNamespaceAlias, PhpNamespacePath, PhpUseImport,
};
#[cfg(feature = "symbol")]
pub use use_php_symbol::{PhpSymbol, SymbolKind, SymbolName};
#[cfg(feature = "syntax")]
pub use use_php_syntax::{PhpDeclarationKind, PhpKeyword, PhpModifier, PhpVisibility};
#[cfg(feature = "token")]
pub use use_php_token::{PhpToken, PhpTokenCategory, PhpTokenSpan, PhpTokenText};
#[cfg(feature = "type")]
pub use use_php_type::{PhpScalarType, PhpType, PhpTypeKind, PhpTypeName};
#[cfg(feature = "version")]
pub use use_php_version::{PhpSupportPhase, PhpVersion, PhpVersionBranch};
#[cfg(feature = "psr")]
pub use use_psr::{PsrCategory, PsrMetadata, PsrNumber, PsrStatus, PsrTitle};
#[cfg(feature = "wordpress")]
pub use use_wordpress::{
    WordPressCapability, WordPressPluginHeader, WordPressPostTypeSlug, WordPressRestRoute,
};
#[cfg(feature = "wordpress-block")]
pub use use_wordpress_block::{WordPressBlockJson, WordPressBlockName, WordPressBlockSupport};
#[cfg(feature = "wordpress-hook")]
pub use use_wordpress_hook::{
    HookCallbackMetadata, HookKind, HookName, HookPriority, HookReference,
};

/// Common RustUse PHP primitive re-exports.
pub mod prelude {
    #[cfg(feature = "composer-json")]
    pub use use_composer_json::{
        ComposerAutoloadConfig, ComposerJson, ComposerPackageName, ComposerPackageType,
        ComposerRequirement, ComposerStability,
    };
    #[cfg(feature = "drupal")]
    pub use use_drupal::{
        DrupalConfigObjectName, DrupalEntityTypeId, DrupalModuleName, DrupalPermission,
        DrupalRouteName, DrupalThemeName,
    };
    #[cfg(feature = "laravel")]
    pub use use_laravel::{
        ArtisanCommandName, LaravelConfigKey, LaravelMetadata, LaravelMiddlewareName,
        LaravelMigrationName, LaravelRouteName, ServiceProviderName,
    };
    #[cfg(feature = "packagist")]
    pub use use_packagist::{
        PackagistDownloadCount, PackagistMetadataLabel, PackagistPackageName, PackagistPackageType,
        PackagistStability,
    };
    #[cfg(feature = "autoload")]
    pub use use_php_autoload::{
        AutoloadConfig, AutoloadPath, AutoloadStrategy, ClassmapEntry, FilesAutoloadEntry,
        Psr4Mapping, Psr4Prefix,
    };
    #[cfg(feature = "docblock")]
    pub use use_php_docblock::{
        Docblock, DocblockTag, DocblockTagKind, DocblockTypeString, TagName,
    };
    #[cfg(feature = "error")]
    pub use use_php_error::{
        DiagnosticMessage, DiagnosticSource, PhpDiagnostic, PhpErrorKind, PhpErrorLevel,
        PhpSeverity,
    };
    #[cfg(feature = "extension")]
    pub use use_php_extension::{
        PhpExtensionKind, PhpExtensionName, PhpExtensionRequirement, PhpExtensionRequirementKind,
        PhpVersionConstraint,
    };
    #[cfg(feature = "ini")]
    pub use use_php_ini::{
        PhpIniDirective, PhpIniDirectiveName, PhpIniEnvironment, PhpIniSectionName, PhpIniValue,
    };
    #[cfg(feature = "namespace")]
    pub use use_php_namespace::{
        GlobalNamespace, PhpFullyQualifiedName, PhpNamespaceAlias, PhpNamespacePath,
        PhpRelativeName, PhpUseImport,
    };
    #[cfg(feature = "symbol")]
    pub use use_php_symbol::{PhpClassLikeKind, PhpMemberKind, PhpSymbol, SymbolKind, SymbolName};
    #[cfg(feature = "syntax")]
    pub use use_php_syntax::{
        PhpControlFlowLabel, PhpDeclarationKind, PhpKeyword, PhpModifier, PhpVisibility,
    };
    #[cfg(feature = "token")]
    pub use use_php_token::{
        PhpCommentKind, PhpDelimiter, PhpLiteralKind, PhpOperator, PhpToken, PhpTokenCategory,
        PhpTokenSpan, PhpTokenText,
    };
    #[cfg(feature = "type")]
    pub use use_php_type::{
        PhpClassLikeTypeName, PhpScalarType, PhpType, PhpTypeKind, PhpTypeName,
    };
    #[cfg(feature = "version")]
    pub use use_php_version::{PhpSupportPhase, PhpVersion, PhpVersionBranch};
    #[cfg(feature = "psr")]
    pub use use_psr::{PsrCategory, PsrMetadata, PsrNumber, PsrStatus, PsrTitle};
    #[cfg(feature = "wordpress")]
    pub use use_wordpress::{
        WordPressCapability, WordPressPluginHeader, WordPressPluginHeaderName,
        WordPressPostTypeSlug, WordPressRestNamespace, WordPressRestRoute, WordPressTaxonomySlug,
        WordPressText,
    };
    #[cfg(feature = "wordpress-block")]
    pub use use_wordpress_block::{
        WordPressBlockAttribute, WordPressBlockAttributeName, WordPressBlockAttributeType,
        WordPressBlockJson, WordPressBlockName, WordPressBlockSupport,
    };
    #[cfg(feature = "wordpress-hook")]
    pub use use_wordpress_hook::{
        HookCallbackMetadata, HookKind, HookName, HookPriority, HookReference,
    };
}
