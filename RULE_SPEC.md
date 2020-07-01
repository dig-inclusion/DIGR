# Rule specification WIP

## Spec

### name (String)

The name of the test, simple as possible.
eg) Img has an alt and it is not blank

### Rule type: atomic/ composite

This would allow us to organise rules and re-use the atomic ones

#### Description 

A short description of the issue

### Accessibility requirements mapping (essentially accessibility metadata)

This section would map to WCAG criteria, WCAG techniques and failures, DIG rules, ACT rules etc. 

### Scope (string | array<string>, Required)

Which element to apply this ruleset to possible values are tag name or selector
eg) `img` or `*[role='img']` or `a[aria-label]`. This section should also correspond with an elements spec (HTML living standard); therefore, I also see some metadata in here. It would also specify if the element to be tested can be hidden.


#### if (Optional)
looping through, runs this test to see if this rule applies, `if` alone is `ifEquals`
variants: ifEquals, ifNotEquals, ifGreaterThan, ifLessThan, ifGreaterThanOrEquals, ifLessThanOrEquals, ifNull, ifNotNull

#### let (object, Optional)
variables to assign for this test, for keeping things tidy

#### assert
The assertion to make to see if the element passes this test, `assert` alone is `assertEquals`
variants: assertEquals, assertNotEquals, assertGreaterThan, assertLessThan, assertGreaterThanOrEquals, assertLessThanOrEquals, assertNull, assertNotNull

### validation (array<object>, Optional)
Rule can be used to test / validate itself

### assert (string(pass, fail, na), Required)
Assertion of result, can either be a pass, a fail or na if the rule did not apply

## Additional notation

`$` can be used to prefix variables, either default or defined in `let`
- $innerText: inner text of this element
- $count: can be used to count elements given a query eg `$count{*[innerText=$innerText]}`
- $element: the tag of this element
- $attributes: can be used to select value of an attribute eg `$attributes[alt]`

## Example:

```
### Non-embeded HTML document human language is determined programmatically

#### Rule type: atomic

#### Description 

This rule checks that each non-embeded HTML document has a default human language specified on the [html element](https://html.spec.whatwg.org/multipage/semantics.html#the-html-element) by using the [lang attribute](https://html.spec.whatwg.org/multipage/dom.html#the-lang-and-xml:lang-attributes) with the value corresponding with a valid [primary language tag from the language tag registry](https://www.iana.org/assignments/language-subtag-registry/language-subtag-registry).

### Accessibility requirements mapping (essentially accessibility metadata)

- [Language of Page 3.1.1](https://www.w3.org/TR/WCAG21/#language-of-page) 
- [Element with lang attribute has valid language tag](https://act-rules.github.io/rules/de46e4#valid-language-tag) 


### Scope (string | array<string>, Required)

This rule applies to a non-embeded html element.

    TEST- this would rely on atomic rules

- name: html element has a language attribute
    let: #variables to use for this test
      html: getElementsByTagName(html) / or whatever else is more suitable
    assertNotGreaterThan: [$html, 1]
  

validation:
  - name: has lang attribute and the attribute value is valid
    case: |
      
        <html lang="cy" ></html>

    assert: pass
  
  - name: Differentiated
    case: |
      <html lang="" ></html>
    <html></html>
    assert: pass
```
