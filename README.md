# swc-transform-imports-extend

swc plugin for babel-plugin-transform-imports and extend

# example

```json
{
    "jsc": {
        "parser": {
            "syntax": "ecmascript"
        },
        "target": "es5",
        "experimental": {
            "plugins": [
                [
                    "swc-transform-imports-extend",
                    {
                        "@hahazexia/my-ui-name": {
                            "casetype": "lowercase",
                            "transform": "@hahazexia/my-ui-name/lib/{{member}}",
                            "style": "@hahazexia/my-ui-name/lib/{{member}}/style/index.css",
                            "skipDefaultConversion": true,
                            "sideEffectPosition": "after"
                        },
                        "@hahazexia/my-component": {
                            "transform": "",
                            "preset": {
                                "jsPath": {
                                    "SomeComponent": "./some-component",
                                },
                                "cssPath": {
                                    "SomeComponent": "./some-component/style/index.css",
                                }
                            }
                        }
                    }
                ]
            ]
        }
    },
    "minify": false
}
```

use .swcrc config above, then if your code is like this:

```js
import { Button } from "@hahazexia/my-ui-name";
import { SomeComponent } from "@hahazexia/my-component";
```

will compile to this:

```js
import { SomeComponent } from "@hahazexia/my-component/lib/some-component";
import "@hahazexia/my-component/lib/some-component/style/index.css";
import { Button } from "@hahazexia/my-ui-name/lib/button";
import "@hahazexia/my-ui-name/lib/button/style/index.css";
```

the style parameter has another form of array, it will iterate over the items and handle all the css import

```json

{
    "jsc": {
        "parser": {
            "syntax": "ecmascript"
        },
        "target": "es5",
        "experimental": {
            "plugins": [
                [
                    "swc-transform-imports-extend",
                    {
                        "@hahazexia/my-ui-name": {
                            "casetype": "lowercase",
                            "transform": "@hahazexia/my-ui-name/lib/{{member}}",
                            "style": [
                                ["^\\w+$", "@hahazexia/my-ui-name/lib/{{member}}/style/index.css"],
                                ["^\\w+$", "@hahazexia/my-ui-name/lib/{{member}}/style/dark.css"]
                            ],
                            "skipDefaultConversion": true,
                            "sideEffectPosition": "after"
                        }
                    }
                ]
            ]
        }
    },
    "minify": false
}
```

then if your code is like this:

```js
import { Button } from "@hahazexia/my-ui-name";
```

will compile to this:

```js
import { Button } from "@hahazexia/my-ui-name/lib/button";
import "@hahazexia/my-ui-name/lib/button/style/index.css";
import "@hahazexia/my-ui-name/lib/button/style/dark.css";
```

