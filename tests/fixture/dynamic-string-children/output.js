import { template as _$template } from "r-dom";
import { insert as _$insert } from "r-dom";
const _tmpl$ = /*#__PURE__*/ _$template(`<div>Hello <!>!</div>`, 3);
const hello = ({ name  })=>{
    return (()=>{
        const _el$ = _tmpl$.cloneNode(true), el$ = _el$.firstChild, _el$1 = el$.nextSibling, el$1 = _el$1.nextSibling;
        _$insert(_el$, name, _el$1);
        return _el$;
    })();
};