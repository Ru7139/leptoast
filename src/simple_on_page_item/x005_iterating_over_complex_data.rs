use leptos::prelude::*;

#[derive(Debug, Clone)]
struct DatabaseEntry {
    key: String,
    value: i32,
}
impl DatabaseEntry {
    fn new(key_str: &str, value: i32) -> DatabaseEntry {
        DatabaseEntry {
            key: key_str.into(),
            value,
        }
    }
}

#[component]
pub fn ComplexDataIter() -> impl IntoView {
    let (data, set_data) = signal(Vec::from([
        DatabaseEntry::new("aeiot", 15),
        DatabaseEntry::new("bezwy", 32),
        DatabaseEntry::new("cothey", 7),
    ]));

    view! {
        <p> "---> ComplexDataIter()" </p>
        <button on:click = move |_| {
            set_data.update(|data| {
                for row in data { row.value += 2; }
            });
            leptos::logging::log!("{:?}", data.get());
        }> "Update Values +2" </button>

        <For
            each = move || data.get()
            key = |state| (state.key.clone(), state.value) // 这里有其他两种写法，这种效率最低
            let(child) // 每次值发生变化时会丢弃之前的<p>，再重新渲染，这导致效率与UI复杂程度成反比
        >
            <p>{child.value}</p>
        </For>

    }
}

#[derive(Debug, Clone)]
struct BasementData<T: Send + Sync + 'static> {
    name: String,
    subfec: RwSignal<T>,
}
impl<T: Send + Sync + 'static> BasementData<T> {
    fn new(name_str: &str, val: T) -> BasementData<T> {
        BasementData {
            name: name_str.into(),
            subfec: RwSignal::new(val),
        }
    }
}

#[component]
pub fn BetterIterA() -> impl IntoView {
    let (data, _set_data) = signal(Vec::<BasementData<i32>>::from([
        BasementData::new("Hoak", 22),
        BasementData::new("Poyoz", 45),
        BasementData::new("Menduet", 10),
    ]));

    view! {
        <p> "---> BetterIterA()" </p>
        <button on:click = move |_| {
            for row in &*data.read() {
                row.subfec.update(|x| *x += 3);
            }
            leptos::logging::log!("{:?}", data.get())
        }> "Update Values +3" </button>

        <For
            each = move || data.get()
            key = |state| state.name.clone() // 这种方式很有效
            let(child) // 缺点在于如果无法控制接收到的数据类型时，可能造成麻烦

        >
        <p> {child.subfec} </p>
        </For>
    }
}

#[component]
pub fn BetterIterMemo() -> impl IntoView {
    type Bi32 = BasementData<i32>;
    let (data, _set_data): (ReadSignal<Vec<Bi32>>, WriteSignal<Vec<Bi32>>) =
        signal(Vec::<BasementData<i32>>::from([
            BasementData::new("Hoak", 22),
            BasementData::new("Poyoz", 45),
            BasementData::new("Menduet", 10),
        ]));

    view! {
        <p> "---> BetterIterMemo()" </p>
        <button on:click = move |_| {
            for row in &*data.read() {
                row.subfec.update(|x| *x += 4);
            }
            leptos::logging::log!("{:?}", data.get())
        }> "Update Values +4" </button>

        <For
            each = move || data.get().into_iter().enumerate()
            key = |(_, state)| state.name.clone() // 当data变化时，Memo会重新计算
            children = move |(index, bacet)| { // 当data内被删除一个对象，Memo也会立即触发一次
                let value = Memo::new(move |_| { // 如果此处的计算量过大，则应尽可能使用反应信号嵌套
                    data.with(|vec_basement|
                        vec_basement.get(index)
                            .map(|d| d.subfec)
                            .unwrap_or(RwSignal::new(-1i32)))
                });
                view! {
                    <p> {bacet.name}": "{value} </p>
                }
            }
        />
    }
}
        <h3> "---> MaybeBestIterStores()" </h3>
