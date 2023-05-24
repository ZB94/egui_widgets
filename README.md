# egui_widgets

## OptionValue

```rust
use egui_widgets::OptionValue;

ui.add(OptionValue::new_full(
    &mut self.option_value,
    "Option String",
    |ui, value| ui.text_edit_singleline(value).changed(),
    || "default value".to_string(),
));
```

![](images/option_value.gif)


## ListViewer

用于展示只读的列表。[示例](examples/list_viewer.rs)

![](images/list_viewer.gif)


## ListEditor

用于编辑列表。[示例](examples/list_editor.rs)

![](images/list_editor.gif)


## EguiTracing

捕获日志并显示。用简单筛选功能。[示例](examples/tracing.rs)

![](images/tracing.gif)
