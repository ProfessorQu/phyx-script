use std::collections::HashMap;

use nannou::color::*;

use crate::runtime::values::RuntimeValue;

pub struct Environment {
    variables: HashMap<String, RuntimeValue>
}

impl Environment {
    pub fn new() -> Self {
        let mut env = Self {
            variables: HashMap::new()
        };

        env.declare_var("true".to_string(), RuntimeValue::Boolean(true)).expect("'true' already declared");
        env.declare_var("false".to_string(), RuntimeValue::Boolean(false)).expect("'false' already declared");

        env.declare_color("aliceblue", ALICEBLUE);
        env.declare_color("antiquewhite", ANTIQUEWHITE);
        env.declare_color("aqua", AQUA);
        env.declare_color("aquamarine", AQUAMARINE);
        env.declare_color("azure", AZURE);
        env.declare_color("beige", BEIGE);
        env.declare_color("bisque", BISQUE);
        env.declare_color("black", BLACK);
        env.declare_color("blanchedalmond", BLANCHEDALMOND);
        env.declare_color("blue", BLUE);
        env.declare_color("blueviolet", BLUEVIOLET);
        env.declare_color("brown", BROWN);
        env.declare_color("burlywood", BURLYWOOD);
        env.declare_color("cadetblue", CADETBLUE);
        env.declare_color("chartreuse", CHARTREUSE);
        env.declare_color("chocolate", CHOCOLATE);
        env.declare_color("coral", CORAL);
        env.declare_color("cornflowerblue", CORNFLOWERBLUE);
        env.declare_color("cornsilk", CORNSILK);
        env.declare_color("crimson", CRIMSON);
        env.declare_color("cyan", CYAN);
        env.declare_color("darkblue", DARKBLUE);
        env.declare_color("darkcyan", DARKCYAN);
        env.declare_color("darkgoldenrod", DARKGOLDENROD);
        env.declare_color("darkgray", DARKGRAY);
        env.declare_color("darkgreen", DARKGREEN);
        env.declare_color("darkgrey", DARKGREY);
        env.declare_color("darkkhaki", DARKKHAKI);
        env.declare_color("darkmagenta", DARKMAGENTA);
        env.declare_color("darkolivegreen", DARKOLIVEGREEN);
        env.declare_color("darkorange", DARKORANGE);
        env.declare_color("darkorchid", DARKORCHID);
        env.declare_color("darkred", DARKRED);
        env.declare_color("darksalmon", DARKSALMON);
        env.declare_color("darkseagreen", DARKSEAGREEN);
        env.declare_color("darkslateblue", DARKSLATEBLUE);
        env.declare_color("darkslategray", DARKSLATEGRAY);
        env.declare_color("darkslategrey", DARKSLATEGREY);
        env.declare_color("darkturquoise", DARKTURQUOISE);
        env.declare_color("darkviolet", DARKVIOLET);
        env.declare_color("deeppink", DEEPPINK);
        env.declare_color("deepskyblue", DEEPSKYBLUE);
        env.declare_color("dimgray", DIMGRAY);
        env.declare_color("dimgrey", DIMGREY);
        env.declare_color("dodgerblue", DODGERBLUE);
        env.declare_color("firebrick", FIREBRICK);
        env.declare_color("floralwhite", FLORALWHITE);
        env.declare_color("forestgreen", FORESTGREEN);
        env.declare_color("fuchsia", FUCHSIA);
        env.declare_color("gainsboro", GAINSBORO);
        env.declare_color("ghostwhite", GHOSTWHITE);
        env.declare_color("gold", GOLD);
        env.declare_color("goldenrod", GOLDENROD);
        env.declare_color("gray", GRAY);
        env.declare_color("grey", GREY);
        env.declare_color("green", GREEN);
        env.declare_color("greenyellow", GREENYELLOW);
        env.declare_color("honeydew", HONEYDEW);
        env.declare_color("hotpink", HOTPINK);
        env.declare_color("indianred", INDIANRED);
        env.declare_color("indigo", INDIGO);
        env.declare_color("ivory", IVORY);
        env.declare_color("khaki", KHAKI);
        env.declare_color("lavender", LAVENDER);
        env.declare_color("lavenderblush", LAVENDERBLUSH);
        env.declare_color("lawngreen", LAWNGREEN);
        env.declare_color("lemonchiffon", LEMONCHIFFON);
        env.declare_color("lightblue", LIGHTBLUE);
        env.declare_color("lightcoral", LIGHTCORAL);
        env.declare_color("lightcyan", LIGHTCYAN);
        env.declare_color("lightgoldenrodyellow", LIGHTGOLDENRODYELLOW);
        env.declare_color("lightgray", LIGHTGRAY);
        env.declare_color("lightgreen", LIGHTGREEN);
        env.declare_color("lightgrey", LIGHTGREY);
        env.declare_color("lightpink", LIGHTPINK);
        env.declare_color("lightsalmon", LIGHTSALMON);
        env.declare_color("lightseagreen", LIGHTSEAGREEN);
        env.declare_color("lightskyblue", LIGHTSKYBLUE);
        env.declare_color("lightslategray", LIGHTSLATEGRAY);
        env.declare_color("lightslategrey", LIGHTSLATEGREY);
        env.declare_color("lightsteelblue", LIGHTSTEELBLUE);
        env.declare_color("lightyellow", LIGHTYELLOW);
        env.declare_color("lime", LIME);
        env.declare_color("limegreen", LIMEGREEN);
        env.declare_color("linen", LINEN);
        env.declare_color("magenta", MAGENTA);
        env.declare_color("maroon", MAROON);
        env.declare_color("mediumaquamarine", MEDIUMAQUAMARINE);
        env.declare_color("mediumblue", MEDIUMBLUE);
        env.declare_color("mediumorchid", MEDIUMORCHID);
        env.declare_color("mediumpurple", MEDIUMPURPLE);
        env.declare_color("mediumseagreen", MEDIUMSEAGREEN);
        env.declare_color("mediumslateblue", MEDIUMSLATEBLUE);
        env.declare_color("mediumspringgreen", MEDIUMSPRINGGREEN);
        env.declare_color("mediumturquoise", MEDIUMTURQUOISE);
        env.declare_color("mediumvioletred", MEDIUMVIOLETRED);
        env.declare_color("midnightblue", MIDNIGHTBLUE);
        env.declare_color("mintcream", MINTCREAM);
        env.declare_color("mistyrose", MISTYROSE);
        env.declare_color("moccasin", MOCCASIN);
        env.declare_color("navajowhite", NAVAJOWHITE);
        env.declare_color("navy", NAVY);
        env.declare_color("oldlace", OLDLACE);
        env.declare_color("olive", OLIVE);
        env.declare_color("olivedrab", OLIVEDRAB);
        env.declare_color("orange", ORANGE);
        env.declare_color("orangered", ORANGERED);
        env.declare_color("orchid", ORCHID);
        env.declare_color("palegoldenrod", PALEGOLDENROD);
        env.declare_color("palegreen", PALEGREEN);
        env.declare_color("paleturquoise", PALETURQUOISE);
        env.declare_color("palevioletred", PALEVIOLETRED);
        env.declare_color("papayawhip", PAPAYAWHIP);
        env.declare_color("peachpuff", PEACHPUFF);
        env.declare_color("peru", PERU);
        env.declare_color("pink", PINK);
        env.declare_color("plum", PLUM);
        env.declare_color("powderblue", POWDERBLUE);
        env.declare_color("purple", PURPLE);
        env.declare_color("rebeccapurple", REBECCAPURPLE);
        env.declare_color("red", RED);
        env.declare_color("rosybrown", ROSYBROWN);
        env.declare_color("royalblue", ROYALBLUE);
        env.declare_color("saddlebrown", SADDLEBROWN);
        env.declare_color("salmon", SALMON);
        env.declare_color("sandybrown", SANDYBROWN);
        env.declare_color("seagreen", SEAGREEN);
        env.declare_color("seashell", SEASHELL);
        env.declare_color("sienna", SIENNA);
        env.declare_color("silver", SILVER);
        env.declare_color("skyblue", SKYBLUE);
        env.declare_color("slateblue", SLATEBLUE);
        env.declare_color("slategray", SLATEGRAY);
        env.declare_color("slategrey", SLATEGREY);
        env.declare_color("snow", SNOW);
        env.declare_color("springgreen", SPRINGGREEN);
        env.declare_color("steelblue", STEELBLUE);
        env.declare_color("tan", TAN);
        env.declare_color("teal", TEAL);
        env.declare_color("thistle", THISTLE);
        env.declare_color("tomato", TOMATO);
        env.declare_color("turquoise", TURQUOISE);
        env.declare_color("violet", VIOLET);
        env.declare_color("wheat", WHEAT);
        env.declare_color("white", WHITE);
        env.declare_color("whitesmoke", WHITESMOKE);
        env.declare_color("yellow", YELLOW);
        env.declare_color("yellowgreen", YELLOWGREEN);

        env
    }

    pub fn declare_var(&mut self, varname: String, value: RuntimeValue) -> Result<RuntimeValue, String> {
        if self.variables.contains_key(&varname) {
            return Err(format!("Cannot declare variable '{:?}' as it's already defined", varname))
        }

        self.variables.insert(varname, value.clone());
        Ok(value)
    }

    pub fn declare_color(&mut self, varname: &str, color: Rgb<u8>) {
        self.declare_var(varname.to_string(), RuntimeValue::Color(color)).unwrap_or_else(|_| panic!("'{:?}' already defined", varname));
    }

    pub fn assign_var(&mut self, varname: String, value: RuntimeValue) -> Result<RuntimeValue, String> {
        let env = self.resolve_mut(&varname)?;
        env.variables.insert(varname, value.clone());

        Ok(value)
    }

    pub fn lookup_var(&self, varname: String) -> Result<RuntimeValue, String> {
        let env = self.resolve(&varname)?;

        Ok(env.variables.get(&varname).expect("'resolve' succeeded but varname is not present").clone())
    }

    pub fn resolve(&self, varname: &String) -> Result<&Environment, String> {
        if self.variables.contains_key(varname) {
            Ok(self)
        } else {
            Err(format!("Failed to resolve variable '{:?}'", varname))
        }
    }

    pub fn resolve_mut(&mut self, varname: &String) -> Result<&mut Environment, String> {
        if self.variables.contains_key(varname) {
            Ok(self)
        } else {
            Err(format!("Failed to resolve variable '{:?}'", varname))
        }
    }
}
