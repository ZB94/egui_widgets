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


# SelectEdit

```rust
use egui_widgets::SelectEdit;

ui.add(SelectEdit::new(
    &mut self.text,
    ('a'..='z')
        .enumerate()
        .map(|c| c.1.to_string().repeat(c.0 + 1)),
));

ui.add(
    SelectEdit::new(
        &mut self.text,
        ('a'..='z')
            .enumerate()
            .map(|c| c.1.to_string().repeat(c.0 + 1)),
    )
    .filter(),
);
```

![](images/select_edit.gif)

## ListView

用于展示只读的列表。[示例](examples/list_view.rs)

![](images/list_view.gif)


## ListEdit

用于编辑列表。[示例](examples/list_edit.rs)

![](images/list_edit.gif)


## EguiTracing

捕获日志并显示。用简单筛选功能。[示例](examples/tracing.rs)

![](images/tracing.gif)
