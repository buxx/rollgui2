function on_init() {
}

register_plugin = function (importObject) {
    importObject.env.present_singleline_text_input = present_singleline_text_input;
    importObject.env.try_recv_text_input = try_recv_text_input;
}

miniquad_add_plugin({ register_plugin, on_init, version: "0.1.0", name: "rollgui2" });

var ongoing_text_input_value = null;

function try_recv_text_input() {
    if (ongoing_text_input_value != null) {
        console.debug("Text input value, return it");
        let value_to_return = ongoing_text_input_value;
        ongoing_text_input_value = null;
        console.debug(value_to_return);
        return js_object(value_to_return)
    }

    console.debug("No text input value, return -1")
    return -1;
}

function present_singleline_text_input(title, value) {
    console.debug("Present single line text input");
    var title_string = consume_js_object(title);
    var value_string = consume_js_object(value);
    let input_value = prompt(title_string, value_string);

    console.debug("Text input value given, save it");
    ongoing_text_input_value = input_value
}
