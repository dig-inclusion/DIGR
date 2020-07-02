# Rule specification WIP

## Spec

### name (String)

The name of the test, simple as possible.
eg) Img has an alt and it is not blank

### meta (Object, Optional)

Optional meta data for the test, included in the output. Can be used to attach any static variables to the output for mapping to other software etc.

### on (string | array<string>, Required)

Which element to apply this ruleset to possible values are tag name or selector
eg) `img` or `*[role='img']` or `a[aria-label]`

### includeHidden (boolean, Optional)

defaults to false if not included. Defines whether to include accessibly hidden elements (aria-hidden, display: none)

### tests (array<object>, Required)

The tests to run in, for each test it loops through all elements selected by `on`
Runs in order: if > let > assert

#### name (string, Required)
Simple name of test for output

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

#### name (string, Required)
Name of test

#### case (string(html), Required)
HTML to test against

### assert (string(pass, fail, na), Required)
Assertion of result, can either be a pass, a fail or na if the rule did not apply

## Additional notation

`$` can be used to prefix variables, either default or defined in `let`
- $innerText: inner text of this element
- $count: can be used to count elements given a query eg `$count{*[innerText=$innerText]}`
- $element: the tag of this element
- $attributes: can be used to select value of an attribute eg `$attributes[alt]`

## Examples:

### Example 1

```
#Optional
#The name of the test, simple as possible, shows for versbose.
#If omitted sha of rule is used?
name: Img has an alt and it is not blank

#Optional
#meta data associated with the test
meta:
  #act-ref: #string of act ID to map to
  dig-ref: DIG001 #string of dig ref to map to

#Required
#selector to get elements, can be string or array
#role selector? will this work?
on: [img, "*[role='img']"]

#Optional
#whether to include accessibly hidden elements (aria-hidden, display: none)
#defaults to false
includeHidden: false

#Required
#how should these tests look?
tests:
  - name: blank
    if: [$element, img] #if to check this test applies #Order: runs if then let then everything else
    assertNotNull: $attributes[alt] #assertion to run

  - name: empty
    if: [$element, img]
    assertNotEquals: ["$attributes[alt]", ""]

  - name: arialabel
    ifNotEquals: [$element, img] #not sure on this?
    assertNotNull: $attributes[aria-label]
    assertNotEquals: ["$attributes[aria-label]", ""]

  - name: arialabelledby
    let:
      foundIds: $count{*[id="$attributes[aria-labelledby]"]}
    ifNotEquals: [$element, img] #not sure on this?
    assertNotNull: $attributes[aria-labelledby]
    assertNotEquals: ["$attributes[aria-labelledby]", ""]
    assertEquals: [$foundIds, 1]

#Required
#Tests to validate the rule, the rule can test itself against itself
validation:
  - name: Standard - Blank #name of the validation test, human readable for debugging
    case: <img src="image.png" /> #HTML code to test
    assert: fail #Asserted result. should this be true/false or pass, fail, na . relies on whether we're including irrelevant examples

  - name: Standard - Empty
    case: <img src="image.png" alt="" />
    assert: fail

  - name: Standard - Correct
    case: <img src="image.png" alt="An orange on a table" />
    assert: pass

  - name: Role - Blank
    case: <span role="img"></span>
    assert: fail

  - name: Role - aria-label
    case: <span role="img" aria-label="An orange on a table"></span>
    assert: pass

  - name: Role - caption
    case: |
      <span role="img" aria-labelledby="label">
        <p id="label">An orange on a table</p>
      </span>
    assert: pass

  - name: Not relevant
    case: <p alt="">Irrelevant example</p>
    assert: na
```

### Example 2

```
name: Duplicate link text

meta:
  dig-ref: DIG1402

on: [a, "*[role='link']"]

includeHidden: false

tests:
  - name: duplicate
    let: #variables to use for this test
      links: $count{*[innerText=$innerText]}
    assertNotGreaterThan: [$links, 1]


validation:
  - name: Duplicate
    case: |
      <div>
        <a href="#">More information</a>
      </div>
      <div>
        <a href="#">More information</a>
      </div>
    assert: fail

  - name: Differentiated
    case: |
      <div>
        <a href="#">More information about a</a>
      </div>
      <div>
        <a href="#">More information about b</a>
      </div>
    assert: pass
```

### Example 3

```
name: Link text is appropriate

meta:
  dig-ref: DIG1401

on: [a, "*[role='link']"]

includeHidden: false

tests:
  - name: clickhere
    assertNotEquals: [$innerText, 'click here'] #case sensitive?

  - name: arialabel
    ifNotNull: $attributes[aria-label]
    assertNotEquals: ["$attributes[aria-label]", ""]


validation:
  - name: Click here
    case: <a href="#">Click here</a>
    assert: fail

  - name: Empty aria-label
    case: <a href="#" aria-label="">go to github</a>
    assert: fail

  - name: Correct - standard
    case: <a href="#">Go to github</a>
    assert: pass

  - name: Correct - arialabel
    case: <a href="#" aria-label="Go to github"></a>
    assert: pass

  - name: Not relevant
    case: <p alt="">Irrelevant example</p>
    assert: na
```

## Aron's Example:

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
