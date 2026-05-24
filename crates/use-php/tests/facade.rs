#![cfg(feature = "full")]

use use_php::prelude::{
    ArtisanCommandName, AutoloadPath, ComposerJson, ComposerPackageName, DiagnosticMessage,
    DrupalModuleName, HookCallbackMetadata, HookKind, HookName, HookReference, LaravelRouteName,
    PackagistPackageName, PhpDiagnostic, PhpErrorKind, PhpExtensionKind, PhpExtensionName,
    PhpExtensionRequirement, PhpFullyQualifiedName, PhpIniDirective, PhpIniDirectiveName,
    PhpIniValue, PhpKeyword, PhpNamespacePath, PhpScalarType, PhpSeverity, PhpSymbol,
    PhpTokenCategory, PhpTokenSpan, PhpTokenText, PhpType, PhpTypeName, PhpVersion, Psr4Mapping,
    Psr4Prefix, PsrCategory, PsrMetadata, PsrNumber, PsrStatus, PsrTitle, SymbolKind, SymbolName,
    WordPressBlockJson, WordPressBlockName, WordPressCapability, WordPressPostTypeSlug,
};

#[test]
fn facade_reexports_every_child_crate() -> Result<(), Box<dyn std::error::Error>> {
    let version: PhpVersion = "8.3.2".parse()?;
    let token_text = PhpTokenText::new("$value")?;
    let token_span = PhpTokenSpan::new(0, 6)?;
    let symbol = PhpSymbol::new(SymbolKind::Class, SymbolName::new("ExampleController")?);
    let namespace = PhpNamespacePath::new("App\\Http")?;
    let name = PhpFullyQualifiedName::new("\\App\\Http\\ExampleController")?;
    let php_type = PhpType::named(PhpTypeName::new("App\\Dto\\UserData")?);
    let ini = PhpIniDirective::new(
        PhpIniDirectiveName::new("display_errors")?,
        PhpIniValue::Boolean(true),
    );
    let diagnostic = PhpDiagnostic::new(
        PhpErrorKind::Runtime,
        PhpSeverity::Error,
        DiagnosticMessage::new("Undefined variable")?,
    );
    let mapping = Psr4Mapping::new(Psr4Prefix::new("App\\")?).with_path(AutoloadPath::new("src/")?);
    let composer = ComposerJson::new().with_name(ComposerPackageName::new("acme/demo")?);
    let packagist = PackagistPackageName::new("symfony/console")?;
    let psr = PsrMetadata::new(
        PsrNumber::new(4)?,
        PsrTitle::new("Autoloading Standard")?,
        PsrStatus::Accepted,
        PsrCategory::Autoloading,
    );
    let extension = PhpExtensionRequirement::required(PhpExtensionName::new("mbstring")?)
        .with_kind(PhpExtensionKind::Bundled);
    let post_type = WordPressPostTypeSlug::new("book")?;
    let capability = WordPressCapability::new("edit_posts")?;
    let hook = HookReference::new(HookName::new("init")?, HookKind::Action)
        .with_callback(HookCallbackMetadata::new("register_books"));
    let block = WordPressBlockJson::new(WordPressBlockName::new("acme/book-card")?);
    let drupal = DrupalModuleName::new("book_tools")?;
    let laravel_route = LaravelRouteName::new("books.index")?;
    let artisan = ArtisanCommandName::new("books:sync")?;

    assert_eq!(version.major(), 8);
    assert_eq!(PhpTokenCategory::Variable.as_str(), "variable");
    assert_eq!(token_text.as_str(), "$value");
    assert_eq!(token_span.len(), 6);
    assert_eq!(PhpKeyword::Readonly.as_str(), "readonly");
    assert_eq!(symbol.name().as_str(), "ExampleController");
    assert_eq!(namespace.segments(), vec!["App", "Http"]);
    assert_eq!(name.to_string(), "\\App\\Http\\ExampleController");
    assert_eq!(PhpType::scalar(PhpScalarType::String).to_string(), "string");
    assert_eq!(php_type.to_string(), "App\\Dto\\UserData");
    assert_eq!(ini.to_string(), "display_errors = true");
    assert_eq!(diagnostic.severity(), PhpSeverity::Error);
    assert_eq!(mapping.prefix().as_str(), "App\\");
    assert_eq!(composer.name().expect("name").package(), "demo");
    assert_eq!(packagist.vendor(), "symfony");
    assert_eq!(psr.identifier(), "PSR-4");
    assert!(extension.is_required());
    assert_eq!(post_type.as_str(), "book");
    assert_eq!(capability.as_str(), "edit_posts");
    assert_eq!(hook.name().as_str(), "init");
    assert_eq!(block.name().as_str(), "acme/book-card");
    assert_eq!(drupal.as_str(), "book_tools");
    assert_eq!(laravel_route.as_str(), "books.index");
    assert_eq!(artisan.as_str(), "books:sync");
    Ok(())
}
