# `error-stack-macros2` v0.2.1

The second development version of `error-stack-macros2` receives its first patch!

## Fixes

- The `#[allow(single_use_lifetimes)]` attribute included with the generated `impl` blocks is now located **above** user-provided attributes. This means that this attribute, as well as any other attributes that we might add in the future, will no longer override attributes that you specify for your type alongside the `error-stack-macros2` macro.
- The path for the generated call to the `unreachable!` macro is now fully qualified, meaning it can no longer accidentally call a different macro in scope with the same name.

## Performance

- Structs with at least one field with type _never_ (`!`) can now ignore the `display` attribute, as such a struct can never be instantiated.

## Dependencies

All dependencies have been updated to their latest versions, which in this case means performance improvements and bug fixes.

## Previous release notes

If you want to take a look at the notes from previous releases, go to [GitHub Releases](https://github.com/luisfi-dev/error-stack-macros2/releases).
