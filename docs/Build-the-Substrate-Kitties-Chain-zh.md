# Build the Substrate Kitties Chain

<aside>
📅date: 2021/11/23     🙂Substrate version:  monthly-2021-11-1
</aside>

[Substrate_ 原文](https://docs.substrate.io/tutorials/v3/kitties/pt1/)

欢迎来到 **Substrate Kitties** 研讨会，这个研讨会会教你所有你需要知道的关于如何创建一个可以控制  NFTs（non-fungible tokens）的创建和所有权的区块链，我们把它叫做 **Substrate Kitties。**研讨会会分成两部分:

**Part I** 描述了如何创建 **Kitties Pallet**，包括与你所创建的 Kitties 互动的功能。

**Part II** 描述了如何开发一个前端与 **Part I** 中创建的区块链互动。

## 学习目标

- 学习和构建运行 `Substrate node` 的基本方式。
- 编写一个自定义的 FRAME pallet 集成到你的 runtime 中。
- 学习如何创建和更新 storage items。
- 编写 pallet 的 extrinsice 和 helper 函数。
- 使用 `PolkadotJS API` 将前端连接到 `Substrate node`。

<aside>
💡 本教程假设在你的机器已经准备好构建 Substrate 需要的 [Rust and the Rust toolchain](https://docs.substrate.io/v3/getting-started/installation/) 。

</aside>

### **What we're building**

我们会有意让事情保持简单，以便您之后可以决定如何改进 **Substrate Kitties** 。对于我们当前构建的 **Substrate Kitties**， 它实际上只能做以下的事情：

- 由 original source 或现有的 Kitties 创建新的 Kitties。
- 定价并出售 Kitties。
- 转移 Kitties 的拥有权。

### **What we won't cover**

本教程不涉及一下项目：

- 为 pallet 编写测试。
- 使用正确的 weight 值。

完成本次教程后，你可以参考 [how-to guides](https://docs.substrate.io/how-to-guides/v3/) 去学习如何集成这些内容。

按着自己的节奏走每一步 — 目标是让您学习，而最好的方法是自己尝试。在每进入下一个部分的时候确保 pallet 构建没有错误。使用 [template files](https://github.com/substrate-developer-hub/substrate-docs/tree/main/static/assets/tutorials/kitties-tutorial) 帮助您完成每一部分。 如果你卡住了，你可以参考在 [Substrate Node Template repository `tutorials/kitties` branch](https://github.com/substrate-developer-hub/substrate-node-template/tree/tutorials/kitties) 上的完整源代码。 大部分代码变化在 `/pallets/kitties/src/lib.rs` 中。

## **Basic set-up**

在制作 Kitties 之前我们需要先做一些基础工作。本部分会包含使用 https://github.com/substrate-developer-hub/substrate-node-template 设置自定义 pallet 和 包含简单 storage item 的基本模式。

### **Set-up your template node**

https://github.com/substrate-developer-hub/substrate-node-template 为我们提供了一个可定制的区块链节点，包括内置的网络和共识层。我们需要关注的是扩展 runtime 和 pallet 的逻辑。

为了开始工作，我们需要设置我们的项目名称和依赖项。我们将使用一个名为 [`kickstart`](https://github.com/Keats/kickstart) 的 CLI 工具来轻松地重命名我们的节点模板。

1. 运行 `cargo install kickstart` 来安装 `kickstart` 。
2. 安装完成后， 在工作目录下运行以下命令：
    
    ```bash
    kickstart https://github.com/sacha-l/kickstart-substrate
    ```
    
    此命令将克隆最新节点模板的副本，并询问您希望如何调用您的节点和 pallet 。
    
3. 输入
    - kitties - 这个名字会用作 node 的名字 - `node-kitties`。
    - kitties - 这个名字会用作 pallet 的名字 - `pallet-kitties`。
    
    这会创建一个包含 https://github.com/substrate-developer-hub/substrate-node-template 的副本的 `kitties` 文件夹，模板副本中的 node，runtime 和 pallet 的名称也会相应的更改。
    
4. 在代码编辑器中打开 kitties 目录， 并将其重命名为 `kitties-tutorial` 或任何你喜欢的名字，以保持有组织的工作。
    
    请注意 `kickstart` 命令修改的目录:
    
    - `/node/` -该文件夹包含所有 node 允许与 runtime 和 RPC 客户端交互的逻辑。
    - `/pallets/` - 你所有的自定义 pallets 都存放在这里。
    - `/runtime/` - 该文件是为 chain runtime 聚合和实现所有 pallets （包括自定义和外部 pallets） 的地方。
5. 在 `runtime/src/lib.rs` 你会发现 pallet 名称仍然是 `TemplateModule` , 修改为 `SubstrateKitties` 。
    
    ```rust
    construct_runtime!(
        // --snip
    {
            // --snip
    				TemplateModule: pallet_kitties, // -> replace it with the following line.
            SubstrateKitties: pallet_kitties,
        }
    );
    ```
    

### 编写 `pallet_kitties` 脚手架

让我们看一下工作区的文件夹结构:

```bash
kitties-tutorial           <--  The name of our project directory
|
+-- node
|
+-- pallets
|   |
|   +-- kitties
|       |
|       +-- Cargo.toml
|       |
|       +-- src
|           |
|           +-- benchmarking.rs     <-- Remove file
|           |
|           +-- lib.rs              <-- Remove contents
|           |
|           +-- mock.rs             <-- Remove file
|           |
|           +-- tests.rs            <-- Remove file
|
+-- Cargo.toml
```

> 您可以删除 `benchmarking.rs`、`mock.rs` 和 `tests.rs` 文件。在本教程中，我们不会学习如何使用这些工具。如果你想了解测试是如何工作的，可以看看 *[this how-to guide](https://docs.substrate.io/how-to-guides/v3/testing/transfer-function/)*。
> 

在 Substrate 中 Pallets 用于定义运行时逻辑。在我们的 **Substrate Kitties** 应用中，我们会创建一个 管理所有逻辑的 Pallet 。

请注意 pallet 的目录 `/pallets/kitties/` 和我们 pallet 的名字不一样。我们 pallet 的名字在 Cargo 理解看来是 `pallet-kitties` 。

 每个 FRAME pallet 都有：

- `frame_support` 和 `frame_system` 依赖.
- 必须的 [attribute macros](https://docs.substrate.io/v3/runtime/macros/#frame-macros-and-attributes) （例如 configuration traits, storage items 和 function calls）。

<aside>
💡 随着本教程下一部分的进行，我们将更新更多的依赖项。

</aside>

这是我们在本教程中构建的 Kitties pallet 的最基础版本。它时开始本教程下一部分的起点。就像本教程的帮助文件一样，使用 TODO 注释，来帮助我们理解稍后要写的代码，使用 ACTION 注释，来解释当前部分编写的代码。

1. 粘贴以下代码到 `/pallets/kitties/src/lib.rs`：
    
    ```rust
    #![cfg_attr(not(feature = "std"), no_std)]
    
    pub use pallet::*;
    
    #[frame_support::pallet]
    pub mod pallet {
      use frame_support::{
        dispatch::{DispatchResult, DispatchResultWithPostInfo},
        pallet_prelude::*,
        sp_runtime::traits::{Hash, Zero},
        traits::{Currency, ExistenceRequirement, Randomness},
      };
      use frame_system::pallet_prelude::*;
      use sp_io::hashing::blake2_128;
    
      // TODO Part II: 用于保存 Kitty 信息的 struct
    
      // TODO Part II: 在 Kitty struct 使用 Enum and implementation 来处理 Gender type.
    
      #[pallet::pallet]
      #[pallet::generate_store(pub(super) trait Store)]
      #[pallet::generate_storage_info]
      pub struct Pallet<T>(_);
    
      /// 通过指定 pallet 所依赖的参数和类型来配置 pallet
      #[pallet::config]
      pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    
        /// The Currency handler for the Kitties pallet.
        type Currency: Currency<Self::AccountId>;
    
        // TODO Part II: Specify the custom types for our runtime.
      }
    
      // Errors.
      #[pallet::error]
      pub enum Error<T> {
        // TODO Part III
      }
    
      #[pallet::event]
      #[pallet::generate_deposit(pub(super) fn deposit_event)]
      pub enum Event<T: Config> {
        // TODO Part III
      }
    
      // ACTION: Storage item to keep a count of all existing Kitties.
    
      // TODO Part II: Remaining storage items.
    
      // TODO Part III: Our pallet's genesis configuration.
    
      #[pallet::call]
      impl<T: Config> Pallet<T> {
        // TODO Part III: create_kitty
    
        // TODO Part III: set_price
    
        // TODO Part III: transfer
    
        // TODO Part III: buy_kitty
    
        // TODO Part III: breed_kitty
      }
    
      // TODO Parts II: helper function for Kitty struct
    
      impl<T: Config> Pallet<T> {
        // TODO Part III: helper functions for dispatchable functions
    
        // TODO: increment_nonce, random_hash, mint, transfer_from
      }
    }
    ```
    
2. 请注意，我们在 pallet 中使用了 `sp_io`，确保这在 pallet 的 `Cargo.toml` 文件中声明为依赖项:
    
    ```rust
    [dependencies.sp-io]
    default-features = false
    git = 'https://github.com/paritytech/substrate.git'
    tag = 'monthly-2021-11-1'
    version = '4.0.0-dev'
    ```
    
3. 现在尝试使用一下命令来构建 pallet。我们还不会构建整个链，因为我们还没有在 runtime 实现 Currency 类型。至少我们可以检查到目前为止我们的 pallet 没有错误:
    
    ```bash
    cargo build -p pallet-kitties
    ```
    
    <aside>
    ⚠️ 检测您是否使用了正确的 `monthly-*` tag 和 version， 否则你将得到一个依赖错误。 这里, Substrate 使用的是 `monthly-2021-11-1` tag 。
    
    </aside>
    
    <aside>
    ⚠️ 您会注意到Rust编译器会对未使用的导入发出警告。没关系。忽略它们——我们将在教程的后面部分使用这些导入。
    
    </aside>
    

### **Add storage items**

让我们向 runtime 添加最简单的逻辑:一个在运行时存储变量的函数。为此，我们需要使用来自 `Substrate [storage API](https://docs.substrate.io/rustdocs/latest/frame_support/storage/index.html)`  的 `[StorageValue](https://docs.substrate.io/rustdocs/latest/frame_support/storage/trait.StorageValue.html)` ，这是一个依赖于 [`storage macro`](https://docs.substrate.io/v3/runtime/macros/#palletstorage) 的 trait。

这意味着任何需要声明的 storage item，我们必须事先包含 `#[pallet::storage]` macro。[在此了解有关声明存储项目的更多信息。](https://docs.substrate.io/v3/runtime/storage/#declaring-storage-items)

在 `pallets/kitties/src/lib.rs` 中 使用一下代码替代 ACTION line：

```rust
#[pallet::storage]
#[pallet::getter(fn kitty_cnt)]
/// Keeps track of the number of Kitties in existence.
pub(super) type KittyCnt<T: Config> = StorageValue<_, u64, ValueQuery>;
```

这为我们的 pallet 创建了一个 storage item，以跟踪现有的小猫总数。

### **Add Currency implementation**

在继续构建我们的 node 之前，我们需要将 Currency type 添加到 pallet 的 runtime 实现中， 在 `runtime/src/lib.rs` 添加以下内容：

```rust
impl pallet_kitties::Config for Runtime {
    type Event = Event;
    type Currency = Balances; // <-- Add this line
}
```

现在构建您的节点，并确保没有任何错误。第一次构建需要一点时间。

```bash
cargo build --release
```

🎉***Congratulations!***🎉

您已经完成了本系列的第一部分。在这个阶段，您已经学习了以下各种模式:

- 定制 Substrate Node Template 和其中的 pallet。
- 构建 Substrate chain 并检查目标 pallet 是否可成功编译。
- 声明存储单个 `u64` 值的储存项目。

## **Uniqueness, custom types and storage maps**

你已经准备好学习一些使用 FRAME 开发 pallet 的核心概念了，其中包括编写储存结构和实现任意trait 。

使用下面提供的 [helper code](https://github.com/substrate-developer-hub/substrate-docs/tree/main/static/assets/tutorials/kitties-tutorial/02-create-kitties.rs) 完成每一个步骤，这是接下来的基础。

使用以下代码片段更新 pallet code (如果您不想使用模板代码，请跳过此步骤):

```rust
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use frame_support::{
        sp_runtime::traits::Hash,
        traits::{ Randomness, Currency, tokens::ExistenceRequirement },
        transactional
    };
    use sp_io::hashing::blake2_128;

    #[cfg(feature = "std")]
    use frame_support::serde::{Deserialize, Serialize};

    // ACTION #1: Write a Struct to hold Kitty information.

    // ACTION #2: Enum declaration for Gender.

    // ACTION #3: Implementation to handle Gender type in Kitty struct.

  #[pallet::pallet]
  #[pallet::generate_store(pub(super) trait Store)]
  pub struct Pallet<T>(_);

    /// Configure the pallet by specifying the parameters and types it depends on.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// The Currency handler for the Kitties pallet.
        type Currency: Currency<Self::AccountId>;

        // ACTION #5: Specify the type for Randomness we want to specify for runtime.

        // ACTION #9: Add MaxKittyOwned constant
    }

    // Errors.
    #[pallet::error]
    pub enum Error<T> {
        // TODO Part III
    }

    // Events.
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        // TODO Part III
    }

    #[pallet::storage]
    #[pallet::getter(fn all_kitties_count)]
    pub(super) type KittyCnt<T: Config> = StorageValue<_, u64, ValueQuery>;

    // ACTION #7: Remaining storage items.

    // TODO Part IV: Our pallet's genesis configuration.

    #[pallet::call]
    impl<T: Config> Pallet<T> {

        // TODO Part III: create_kitty

        // TODO Part IV: set_price

        // TODO Part IV: transfer

        // TODO Part IV: buy_kitty

        // TODO Part IV: breed_kitty
    }

    //** Our helper functions.**//

    impl<T: Config> Pallet<T> {

        // ACTION #4: helper function for Kitty struct

        // TODO Part III: helper functions for dispatchable functions

        // ACTION #6: function to randomly generate DNA

        // TODO Part III: mint

        // TODO Part IV: transfer_kitty_to
    }
}
```

除了这些代码，我们还需要导入 serde。将此添加到 pallet 的 cargo.toml 文件中。

```rust
[dependencies.serde]
version =  '1.0.129'
```

### **Scaffold Kitty struct**

Rust 中的 Struct 是储存相关属性的结构，类似于对象。出于我们的目的，我们的 Kitty 会携带多个 properties，我们可以将这些属性储存在一个 Struct 中， 而不是使用单独的 storage item。这在尝试优化读写储存时非常有用，这样我们可以执行更少的读/写来更新多个值。[阅读更多有关存储最佳实践的信息](https://docs.substrate.io/v3/runtime/storage/#best-practices)。

**what information to include**

让我来看看第一只 小Kitty 会携带哪些信息：

- `dna` — 使用 hash 标识 DNA，标识其唯一性。DNA 还有来繁衍新的小猫，用来跟踪不同的小猫世代。
- `price` — 购买小猫所需的金额，由主人决定
- `gender` — `enum` 男性或女性
- `owner` — 一个 account id，代表他的拥有者

**Sketching out the types held by our struct**

从上面的结构项，我们可以推导出以下类型：

- **`[u8; 16]`** for `dna` - 使用 16 bytes 来代表 Kitty 的 DNA。
- **`BalanceOf`** for `price` - 使用 FRAME's `[Currency` trait](https://docs.substrate.io/rustdocs/latest/frame_support/traits/tokens/currency/trait.Currency.html#associatedtype.Balance) 的自定义类型。
- **`Gender`** for `gender` - 性别枚举 enum。

首先，在我们声明我们的结构之前，我们需要为 `BalanceOf` 和 `AccountOf` 添加自定义类型。用以下代码片段替换 ACTION #1:

```rust
type AccountOf<T> = <T as frame_system::Config>::AccountId;
type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

// Struct for holding Kitty information.
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct Kitty<T: Config> {
    pub dna: [u8; 16],
    pub price: Option<BalanceOf<T>>,
    pub gender: Gender,
    pub owner: AccountOf<T>,
}
```

<aside>
⚠️ 我们定义了 `<BalanceOf<T>>` 和 `AccountOf<T>` 类型，并在 `Kitty` 中使用他们。 如果你想知道第一行代码在 Rust 中意味着什么，那就是为 trait  frame_system::Config 的关联类型定义了一个叫 `AccountOf` 的类型别名，泛型类型 T 需要绑定到  trait  frame_system::Config。
关于这种语法的更多信息，请参阅 [Rust book](https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#specifying-placeholder-types-in-trait-definitions-with-associated-types)。

</aside>

注意我们如何使用 `derive macro` 来使 Struct 包含 [various helper traits](https://docs.substrate.io/rustdocs/latest/sp_std/prelude/index.html#traits) 。我们需要添加 `TypeInfo`，因为 `Kitty` 使用到了他。在 pallet 顶部添加以下行：

```rust
use scale_info::TypeInfo;
```

对于 `Gender` 类型，我们需要构建自己的自定义枚举和帮助函数。

### **Write a custom type for `Gender`**

我们现在需要一个定义 Kitty 性别的枚举。要创建它，需要构建以下部分：

- 一个 enum 值， 包含  `Male` 和 `Female`。
- 为我们的 Kitty Struct 实现一个 helper function 。

使用以下代码替换 ACTION #2

```rust
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
  #[scale_info(skip_type_params(T))]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum Gender {
    Male,
    Female,
}
```

请注意  `derive macro`  的使用，它必须在枚举声明之前。这将为 enum 实现一些 trait，已在 runtime 中和其他类型对接。

太好了，我们现在知道如何创建自定义的 Struct 了。但是如何为 Kitty 分配一个初始性别呢？为此，我们需要使用 helper function 来配置它。

在配置 Struct 中，预先设置一个值时有用的。例如，当根据一个函数的返回内容设置一个值的时候。在我们的例子中，我们有一个类似的情况，我们需要配置 Kitty Struct，并使其根据 DNA 设置 Gender。为此，我们创建一个 gen_gender 公共函数，随机返回一个 Gender 值。

### **Implement on-chain randomness**

如果我们想要能够区分这些小猫，我们需要给他们独特的属性！在前面的步骤, 我们使用了还未定义的 `KittyRandomness` ，现在让我们开始吧。

我们使用 `frame_support` 的 `Randomness` trait。它能够产生一个随机种子，我们使用它创造和繁衍独特的小猫。

1. 在 pallet 中添加一个绑定 `Randomness` 的类型。
    
    `frame_support` 的 `Randomness` trait 必须指定一个参数来替换 `Output` 和 `BlockNumber` 泛型。查看 [the documentation](https://docs.substrate.io/rustdocs/latest/frame_support/traits/trait.Randomness.html) 和源代码理解他是如何工作的。处于我们的目的，我们希望 使用了该 trait 的函数的 output 是 `[Blake2 128-bit hash](https://docs.substrate.io/rustdocs/latest/sp_core/hashing/fn.blake2_128.html)`，您会注意到它应该已经在您的工作代码库的顶部声明了。
    
    使用以下代码替换 ACTION #5
    
    ```rust
    type KittyRandomness: Randomness<Self::Hash, Self::BlockNumber>;
    ```
    
2. 在运行时指定实际类型。
    
    鉴于，我们已经在 pallet configuration 中添加了一个新的类型，我们需要在 runtime 中设置它的具体类型。如果我们想改变 `KittyRandomness` 正在使用的算法，只需修改 runtime，而不需要修改 pallet。
    
    为了展示这一点，我们设置 `KittyRandomness` 的值为 [FRAME's `RandomnessCollectiveFlip`](https://docs.substrate.io/rustdocs/latest/pallet_randomness_collective_flip/index.html) 的一个实例。方便的是 runtime 已经有一个 [FRAME's `RandomnessCollectiveFlip`](https://docs.substrate.io/rustdocs/latest/pallet_randomness_collective_flip/index.html) 实例了。
    
    ```rust
    impl pallet_kitties::Config for Runtime {
        type Event = Event;
            type Currency = Balances;
        type KittyRandomness = RandomnessCollectiveFlip; // <-- ACTION: add this line.
    }
    ```
    
3. 生成随机 DNA 
    
    生成 DNA 类似与使用 randomness 随机分配类别。不同的是，我们将使用上一部分中导入的blake2_128。使用以下代码替换 ACTION #6。
    
    ```rust
    fn gen_dna() -> [u8; 16] {
        let payload = (
            T::KittyRandomness::random(&b"dna"[..]).0,
            <frame_system::Pallet<T>>::block_number(),
        );
        payload.using_encoded(blake2_128)
    }
    ```
    

### **Write remaining storage items**

为了轻松的跟踪我们所有的小猫，我们约定使用一个唯一 ID 作为我们储存项目的 global key。这意味着一个唯一的 key 将指向我们的 Kitty 对象（即我们之前声明的结构）。

为了做到这一点，我们需要确保新 Kitty 的 ID 总是唯一的。我们可以用一个新的 storage item `Kitties` 来实现这一点，它将是一个从 ID 到 Kitty 对象的映射。

有了这个对象，我们可以通过简单的检测 `Kitties` 中是否已经包含使用特定 ID 的映射来检查冲突。例如，在一个 dispatchable function 内部我们可以这样来检测：

```rust
ensure!(!<Kitties<T>>::exists(new_id), "This new id already exists");
```

runtime 中需要跟踪的 storage items：

- 独特的资产，像货币和小猫（这将由叫做 Kitties 的 storage map 持有）
- 这些资产的所有权，像 account id（这将由一个叫做 KittiesOwned 的新 storage map 来处理）

巍峨了给 `Kitty` struct 创建一个 storage 实例，我们将会使用 `[StorageMap](https://docs.substrate.io/v3/runtime/storage/#storage-map)` — 一个由 FRAME 提供的 hash-map 。

Kitties storage item 看起来像是这样：

```rust
#[pallet::storage]
#[pallet::getter(fn kitties)]
pub(super) type Kitties<T: Config> = StorageMap<
  _,
  Twox64Concat,
  T::Hash,
  Kitty<T>
>;
```

分配 StorageMap 需要：

- The `[Twox64Concat](https://docs.substrate.io/rustdocs/latest/frame_support/struct.Twox64Concat.html)` hashing algorithm.
- A key of type `T::Hash`.
- A value of type `Kitty<T>`.

`KittiesOwned` 与此类似，只是我们将使用 `BoundedVec` 来跟踪 Kitties 的最大数量，并配置在 runtime/src/lib.rs 文件中。

```rust
#[pallet::storage]
#[pallet::getter(fn kitties_owned)]
/// Keeps track of what accounts own what Kitty.
pub(super) type KittiesOwned<T: Config> = StorageMap<
  _,
  Twox64Concat,
  T::AccountId,
  BoundedVec<T::Hash, T::MaxKittyOwned>,
  ValueQuery
>;
```

轮到你了，复制上面两个代码片段，替换 ACTION #7。

在坚持 pallet 编译之前，我们需要在配置特征中添加一个新的类型MaxKittyOwned，它是一个托盘常量类型(类似于前面步骤中的KittyRandomness)。将 ACTION #9 替换为：

```rust
#[pallet::constant]
type MaxKittyOwned: Get<u32>;
```

最后，我们在 `runtime/src/lib.rs` 中定义 `MaxKittyOwned`  类型。这与我们在 Currency 和 KittyRandomness 中遵循的模式相同，只是需要使用 parameter_types！macro 添加一个 fixed u32 值：

```rust
parameter_types! {              // <- add this macro
    // One can own at most 9,999 Kitties
    pub const MaxKittyOwned: u32 = 9999;
    }

/// Configure the pallet-kitties in pallets/kitties.
impl pallet_kitties::Config for Runtime {
    type Event = Event;
    type Currency = Balances;
    type KittyRandomness = RandomnessCollectiveFlip;
    type MaxKittyOwned = MaxKittyOwned; // <- add this line
}
```

现在时检测你的 Kittes blockchain 编译的好时机！

```rust
cargo build --release
```

遇到困难？对照教程这一部分的[完整帮助代码](https://github.com/substrate-developer-hub/substrate-docs/tree/main/static/assets/tutorials/kitties-tutorial/03-dispatchables-and-events.rs)检查您的解决方案。

## **Dispatchables, events, and errors**

在本教程的前一部分中，我们为管理我们的小猫所有权奠定了基础 — 经管他们还不真实存在！在这一部分中，我们将使用我们已经声明的 storage item 创建一个 Kitty。把事情分解一下，我们会这么写：

- **`create_kitty`**：一个允许账户铸造 Kitty 的 dispatchable 或 publicly 函数。
- **`mint()`**： 一个助手函数，更新 pallt 的存储项目并执行错误检查，由 create_kitty 调用。
- **pallet `Events`**： 使用 FRAME's `#[pallet::event]` 属性。

在这一部分的最后，在检查一切编译无误后，我们将使用 **PolkadotJS Apps UI** 调用我们的 `create_kitty` extrinsic。

如果你感到自信，可以继续建立你的代码库。也可以选择参考我们的[基本代码](https://github.com/substrate-developer-hub/substrate-docs/blob/main/static/assets/tutorials/kitties-tutorial/03-dispatchables-and-events.rs)，他还使用各种ACTION 项目来指导您完成本节。

### **Public and private functions**

了解如何围绕 Kitty pallet 的铸造和受有权管理功能进行编码设计决策是非常重要的。

作为开发人员，我们希望确保我们编写的代码高效切优雅。通常情况下，优化其中一一个，另一个也会受益。我们优化 pallet 这两方面的方法是将 “heavy lifting” 方法提取为一个帮助函数。这也提高了代码的可读性和可重用性。正如我们将看到的，我们可以创建私有函数，这些函数可以被多个 dispatchable 函数调用而不影响安全性。事实上，以这种方式构建可以被认为是一种附加的安全特性。查看这个关于编写和使用 helper 方法的[操作指南](https://docs.substrate.io/how-to-guides/v3/basics/helper-functions/)，了解更多信息。

在开始实现这种方法之前，让我们先描绘一下组合 `dispatchables` 和 `helper` 函数是什么样子。

**`create_kitty`** 是一个 dispatchable function 或 extrinsic：

- 检查原始签名
- 使用签名帐户生成随机哈希
- 使用随机哈希创建一个新的 Kitty
- 调用 private `mint()` 方法

**`mint`** 是私有的 helper function

- 检查该 Kitty 是否存在
- 使用新的 Kitty ID 更新 storage (for all Kitties and for the owner's account)
- 更新 Kitty 总数和新的拥有者
- 存放一个 Event，表示一只小猫已经成功创建

### **Write the `create_kitty` dispatchable**

在 FRAME 中 dispatchable 总是遵循相同的结构。所有 pallet dispatchables 都位于 `impl<T: Config> Pallet<T> {}` 中并要使用 `#[pallet::call]` macro 声明。阅读这些 FRAME macros 的 [documentation](https://docs.substrate.io/v3/runtime/macros/#palletcall) 了解它们是如何工作的。我们在这里需要知道的是，它们是 FRAME 的一个有用特性，可以最大限度地减少为将 pallet 正确集成到一个 Substrate chain's runtime 中而编写的代码。

**Weights**

根据文档中描述的对 `#[pallet::call]` 的要求，每个 dispatchable function 必须有一个相关的 weight 参数。Weights 是开发 Substrate 的一个重要组成部分，为创建一个块时花费的计算量提供了一个安全保护。

[Substrate 的加权系统](https://docs.substrate.io/v3/concepts/weight/)使开发人员在调用每个外部元素之前考虑其计算复杂性。这允许节点考虑最坏情况下的执行时间，避免在运行比指定的时间更长的 extrinsics 时落后与网络。此外，很多情况下权重也与[收费系统](https://docs.substrate.io/v3/runtime/weights-and-fees/)密切相关。

由于这只是一个教程，我们将默认所有权重为100，以保持简单。

假设您现在已经使用本节的帮助文件替换了 pallets/kitties/src/lib.rs 文件的内容，请找到 ACTION #1并使用下面的行完成函数的开头:

```rust
#[pallet::weight(100)]
pub fn create_kitty(origin: OriginFor<T>) -> DispatchResult {
    let sender = ensure_signed(origin)?; // <- add this line
    let kitty_id = Self::mint(&sender, None, None)?; // <- add this line
    // Logging to the console
    log::info!("A kitty is born with ID: {:?}.", kitty_id); // <- add this line

    // ACTION #4: Deposit `Created` event

    Ok(())
}
```

我们不会进入 debug，但是登录到控制台是一个有用的提示，可以确保您的 pallet 按预期运行。为了使用 `log::info`，请将其添加到 `cargo.toml` 文件中：

```rust
[dependencies.log]
default-features = false
version = '0.4.14'
```

### **Write the `mint()` function**

正如我们在上一节编写 `create_kitty` 时所看的，我们需要创建一个将唯一的 kitty 对象写入各种 storage items 的 **`mint`** 函数。

我们的 mint 函数将会接受以下参数：

- **owner** - 类型为 `&T::AccountId` - 这表明 kitty 的主人。
- **dna** - 类型为 ****`Option<[u8; 16]>` - 这指定了 kitty 的 DNA，如果为 `None` 则会随机生成一个 DNA。
- **gender** - 类型为 `Option<Gender>` - 同上

返回类型为`Result<T::Hash, Error<T>>`。

负责下面的代码片段来编写 `mint` 函数，替换 ACTION #2:

```rust
// Helper to mint a Kitty.
pub fn mint(
    owner: &T::AccountId,
    dna: Option<[u8; 16]>,
    gender: Option<Gender>,
) -> Result<T::Hash, Error<T>> {
    let kitty = Kitty::<T> {
        dna: dna.unwrap_or_else(Self::gen_dna),
        price: None,
        gender: gender.unwrap_or_else(Self::gen_gender),
    owner: owner.clone(),
    };

    let kitty_id = T::Hashing::hash_of(&kitty);

    // Performs this operation first as it may fail
    let new_cnt = Self::kitty_cnt().checked_add(1)
        .ok_or(<Error<T>>::KittyCntOverflow)?;
    // Performs this operation first because as it may fail
    <KittiesOwned<T>>::try_mutate(&owner, |kitty_vec| {
        kitty_vec.try_push(kitty_id)
    }).map_err(|_| <Error<T>>::ExceedMaxKittyOwned)?;
    <Kitties<T>>::insert(kitty_id, kitty);
    <KittyCnt<T>>::put(new_cnt);
    Ok(kitty_id)
}
```

让我们看看上面的代码做了什么：

首先我们创建了一个新的 Kitty 对象，然后基于新 kitty 的属性使用 `hashing` 函数创建一个唯一的 kitty_id。

接下来，我们使用 `kitty_cnt` 函数来递增 KittyCnt。并使用 `checked_add` 函数来检查溢出。

完成检查后，我们将通过以下方式更新存储项目:

使用 `try_mutate` 新增账户拥有的 kitty。

使用 Substrate 的 StorageMap API 提供的 `insert` 方法来存储实际的 Kitty 对象，并将其与其 kitty_id 相关联。

使用 StorageValue API `put` 方法来储存最新的 Kitty 数量。

<aside>
💡 快速回顾一下我们的 storage items
`<Kitties<T>>`: 以 kitty_id 为 key 储存 kitty 对象的字典。
`<KittyOwned<T>>`: 跟踪哪些账户拥有哪些小猫。
`<KittyCnt<T>>`: 现存所有小猫的数量。

</aside>

### **Implement pallet `Events`**

我们的 pallet 也可以在函数结束时发出 event。这不仅报告了函数成功执行，还告诉“链外世界”某个特定的状态转换已经发生。

使用 FRAME `#[pallet::event]` 属性帮助我们轻松的管理和声明 pallet events。使用 FRAME macros 创建 events 只需声明一个 enum：

```rust
#[pallet::event]
#[pallet::generate_deposit(pub(super) fn deposit_event)]
pub enum Event<T: Config>{
    /// A function succeeded. [time, day]
    Success(T::Time, T::Day),
}
```

正如您在上面的代码片段中看到的，我们使用 attribute macro：

`#[pallet::generate_deposit(pub(super) fn deposit_event)]`

这允许我们使用以下模式存放特定事件:

```rust
Self::deposit_event(Event::Success(var_time, var_day));
```

为了使用 pallet 中的事件，我们需要在 `pallet's configuration trait Config` 中添加一个新的关联类型 Event。此外就像在 `pallet's Config trait` 中添加任何类型一样，我们需要在 runtime runtime/src/lib.rs 中定义它。

这种模式与我们在本教程前面将 `KittyRandomness` 类型添加到 `pallet's Config trait` 中时相同，并且已经包含在我们的代码库的初始支架中:

```rust
/// Configure the pallet by specifying the parameters and types it depends on.
#[pallet::config]
pub trait Config: frame_system::Config {
  /// Because this pallet emits events, it depends on the runtime's definition of an event.
  type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
  //--snip--//
}
```

通过将 ACTION #3 替换为以下内容来声明 pallet 事件:

```rust
/// A new Kitty was successfully created. \[sender, kitty_id\]
Created(T::AccountId, T::Hash),
/// Kitty price was successfully set. \[sender, kitty_id, new_price\]
PriceSet(T::AccountId, T::Hash, Option<BalanceOf<T>>),
/// A Kitty was successfully transferred. \[from, to, kitty_id\]
Transferred(T::AccountId, T::AccountId, T::Hash),
/// A Kitty was successfully bought. \[buyer, seller, kitty_id, bid_price\]
Bought(T::AccountId, T::AccountId, T::Hash, BalanceOf<T>),
```

在本教程的最后一部分，我们将使用这些事件中的大多数。现在，让我们将相关事件用于`create_kitty`。

将 ACTION #4 替换为以下内容：

```rust
Self::deposit_event(Event::Created(sender, kitty_id));
```

<aside>
⚠️ 如果你正在从上一部分构建您的代码库（并且还没有使用这个部分的帮助文件），你将需要添加 `OK(())` 并正确关闭 `create_kitty` 函数。

</aside>

### **Error handling**

FRAME 为我们提供了一个使用 `[#pallet::error]` 的错误处理系统。该系统允许我们为 pallet 指定错误，并在 pallet 的各个函数中使用它们。

使用 FRAME 提供的 `#[pallet::error]` 宏声明所有可能的错误，将 ACTION #5a 替换为:

```rust
/// Handles arithmetic overflow when incrementing the Kitty counter.
KittyCntOverflow,
/// An account cannot own more Kitties than `MaxKittyCount`.
ExceedMaxKittyOwned,
/// Buyer cannot be the owner.
BuyerIsKittyOwner,
/// Cannot transfer a kitty to its owner.
TransferToSelf,
/// Handles checking whether the Kitty exists.
KittyNotExist,
/// Handles checking that the Kitty is owned by the account transferring, buying or setting a price for it.
NotKittyOwner,
/// Ensures the Kitty is for sale.
KittyNotForSale,
/// Ensures that the buying price is greater than the asking price.
KittyBidPriceTooLow,
/// Ensures that an account has enough funds to purchase a Kitty.
NotEnoughBalance,
```

我们在下一节中编写的函数中将会使用这些错误。请注意，我们已经在 `mint` 函数中使用了 `KittyCntOverflow` 和 `ExceedMaxKittyOwned`。

现在是看看你的链能否编译的好时机。不要只检查 pallet 是否编译，而是运行以下命令来查看是否一切都可以构建:

```rust
cargo build --release
```

<aside>
💡 如果您遇到错误，请滚动到终端中的第一条错误消息，确定哪一行出现错误，并检查您是否正确执行了每一步。有时大括号的不匹配会释放出一大堆难以理解的错误——仔细检查你的代码！

</aside>

🎉***Congratulations!***🎉

这是我们 Kitties pallet 的核心功能。在下一部分中，您将能看到目前为止所构建的一切。

### **Testing with Polkadot-JS Apps UI**

1.  运行您的链，并使用 [PolkadotJS Apps UI](https://polkadot.js.org/apps/#/explorer) 与之交互。在您的项目目录中运行：
    
    ```rust
    ./target/release/node-kitties --tmp --dev
    ```
    
    通过这样做，我们指定在开发人员模式下运行一个临时链，这样就不需要在每次我们想要启动一个新链时清除存储。
    
2. 前往 [Polkadot.js Apps UI](https://polkadot.js.org/apps/#/explorer)。
3. 点击左上角圆形网络图标，打开 “Development” 部分，选择 “Local Node”。您的节点默认为`127.0.0.1.:9944`。
4. 转到："Developer" -> "Extrinsics" 并通过调用 kitty pallet 中的 createKitty 方法签名提交交易。 从爱丽丝、鲍勃和查理的账户中进行3笔不同的交易。
5. 转到 "Network" -> "Explorer" 检查 "Created" 事件。您应该能够看到发出的事件并查询块详情信息。
6. 去 "Developer" -> "Chain State" 查看您新创建的 Kitty 的详细信息。通过选择 Kitties pallet 并查询 `Kitties(Hash): Kitty`。
    
    请务必取消选中“包含选项”框，您应该能够以以下格式查看新 Kitties 的详细信息:
    
    ```rust
    [
      [
        [
          0x7ec106f6bb3892b650e81b7fe400431543965b8fc5feef4698aa37bac1948e8b
        ]
        {
          dna: 0x310919e0677f38d5ac3ddd3558077a86
          price: null
          gender: Female
          owner: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
        }
      ]
    ]
    ```
    
7. 检查其他 storage items 是否正确反映了其他 Kittes 的创建。

## **Interacting with your Kitties**

到目前为止，您已经建立了一个可以创建和跟踪 Kitties 所有权的链。我们希望通过添加买卖小猫的功能让我们的 runtime 更像一个游戏。为了实现这一点，我们需要让用户可以标记和更新他们的小猫的价格。然后我们可以再添加转移，购买和繁殖小猫的功能。

### **Set a price for each Kitty**

在这个部分的[帮助文件](https://github.com/substrate-developer-hub/substrate-docs/tree/main/static/assets/tutorials/kitties-tutorial/04-interacting-functions.rs)中，您会注意到 set_price 的结构已经准备好了。

你的任务就是使用在 A - D 学到的内容填充 ACTION #1a，#1b，#2 和 #3。

**A. Checking Kitty owner**

当我们创建修改Storage 中对象的函数时，我们应该首先确保只有适当的用户才能成功执行函数中的逻辑。

所有权检查的一般模式如下所示:

```rust
let owner = Self::owner_of(object_id).ok_or("No owner for this object")?;

ensure!(owner == sender, "You are not the owner");
```

第一行检查 `Self::owner_of(object_id)` 是否返回一个 `Some(val)`，如果是则转换为 `Result::Ok(val)`，最后从结果中提取出 `val`。如果不是，则使用提供的错误信息将其转换为 `Result::Err()` ，并使用错误对象提前返回。
第二行检查 `owner == sender`。如果为真，则继续往下执行。如果不是则立即使用提供的错误信息将其转换为 `Result::Err()` ，并使用错误对象提前返回。

**你的回合！**

复制以下代码替换 ACTION #1a：

```rust
ensure!(Self::is_kitty_owner(&kitty_id, &sender)?, <Error<T>>::NotKittyOwner);
```

复制以下代码到 ACTION #1b：

```rust
pub fn is_kitty_owner(kitty_id: &T::Hash, acct: &T::AccountId) -> Result<bool, Error<T>> {
    match Self::kitties(kitty_id) {
        Some(kitty) => Ok(kitty.owner == *acct),
        None => Err(<Error<T>>::KittyNotExist)
    }
}
```

粘贴到 ACTION #1b 中的代码实际上是将两个检查组合在一起，如果 *`Self*::kitties(kitty_id)` 为 `None`，则代表小猫不存在。如果存在，则提取 `val.owner` 与 `acct` 比较并返回结果。

而 ACTION #1a 的代码则使用 `?` 操作符，在 `is_kitty_owner` 返回 `<Error<T>>::KittyNotExist` 错误时直接接该错误返回，否则提取出 Result 中的 bool 值，来进行断言判断。

**B. Updating the price of our Kitty object**

每个 Kitty 对象都有一个 price 属性，在本教程前面的 `mint` 函数中，我们将该属性设置为无默认值:

```rust
let kitty = Kitty::<T> {
    dna: dna.unwrap_or_else(Self::gen_dna),
    price: None,                           //<-- 👀 here
    gender: gender.unwrap_or_else(Self::gen_gender),
    owner: owner.clone(),
};
```

要更新 Kitty 的价格，我们需要：

- 从 storage 获取 Kitty 对象
- 更新对象的 price 值
- 将它重新储存到 storage 中

使用以下方式修改已经存在于 storage 中的值

```rust
let mut object = Self::get_object(object_id);
object.value = new_value;

<Object<T>>::insert(object_id, object);
```

<aside>
💡 每当一个 value 将要被更新，rust 希望你将它声明为 mutable（使用 mut 关键字）。

</aside>

复制以下代码到 ACTION #2：

```rust
kitty.price = new_price.clone();
<Kitties<T>>::insert(&kitty_id, kitty);
```

**D. Deposit an Event**

一旦通过所有检查将新价格写入 storage，我们就可以像之前做的一样发出事件。将 ACTION #3 替换为：

```rust
// Deposit a "PriceSet" event.
Self::deposit_event(Event::PriceSet(sender, kitty_id, new_price));
```

现在，每当 `set_price` 被成功调用时，它都会发出一个 `PriceSet` event。

**Transfer a Kitty**

基于我们之前构建的create_kitty函数，您已经拥有了创建 `Transfer` 函数所需的工具和知识。主要区别在于，实现这一点有两个部分:

1. 一个名为 transfer 的可调度函数：这是一个由 pallet 公开的可以公开调用的可调度函数。
2. 叫做 transfer_kitty_to 的私有函数：这是一个由 transfer 调用的私有函数，用于在传输 Kitty 更新 storage。

以这种方式分离逻辑使得私有 `transfer_kitty_to` 函数可以被 pallet 的其他可调度函数重用，而不需要复制代码。在我们的例子中，下一步将要创建的 `buy_kitty` 可调度函数将会重用它。

**`transfer`**

使用以下代码替换 ACTION #4：

```rust
#[pallet::weight(100)]
pub fn transfer(
    origin: OriginFor<T>,
    to: T::AccountId,
    kitty_id: T::Hash
) -> DispatchResult {
    let from = ensure_signed(origin)?;

    // Ensure the kitty exists and is called by the kitty owner
    ensure!(Self::is_kitty_owner(&kitty_id, &from)?, <Error<T>>::NotKittyOwner);

    // Verify the kitty is not transferring back to its owner.
    ensure!(from != to, <Error<T>>::TransferToSelf);

    // Verify the recipient has the capacity to receive one more kitty
    let to_owned = <KittiesOwned<T>>::get(&to);
    ensure!((to_owned.len() as u32) < T::MaxKittyOwned::get(), <Error<T>>::ExceedMaxKittyOwned);

    Self::transfer_kitty_to(&kitty_id, &to)?;

    Self::deposit_event(Event::Transferred(from, to, kitty_id));

    Ok(())
}
```

到现在为止，上面的模式应该已经很熟悉了。我们总是检查交易是否被签署（`ensure_signed`）；然后我们验证:

1. 被转移的小猫是否为交易的发起者所有。
2. 小猫不会被转移给他的主人（一个多余的操作）
3. 接受者有能力再接受一只小猫

最后我们调用 `transfer_kitty_to` 函数，来更新需要更新的 storage items。

**`transfer_kitty_to`**

`transfer_kitty_to` 要执行安全检查并更新以下存储项目：

1. `KittiesOwned`：更新 Kitty owner。
2. `Kitties`：重新将 price 设置为 `None`。

复制以下代码到 ACTION #5：

```rust
#[transactional]
pub fn transfer_kitty_to(
    kitty_id: &T::Hash,
    to: &T::AccountId,
) -> Result<(), Error<T>> {
    let mut kitty = Self::kitties(&kitty_id).ok_or(<Error<T>>::KittyNotExist)?;

    let prev_owner = kitty.owner.clone();

    // Remove `kitty_id` from the KittyOwned vector of `prev_kitty_owner`
    <KittiesOwned<T>>::try_mutate(&prev_owner, |owned| {
        if let Some(ind) = owned.iter().position(|&id| id == *kitty_id) {
            owned.swap_remove(ind);
            return Ok(());
        }
        Err(())
    }).map_err(|_| <Error<T>>::KittyNotExist)?;

    // Update the kitty owner
    kitty.owner = to.clone();
    // Reset the ask price so the kitty is not for sale until `set_price()` is called
    // by the current owner.
    kitty.price = None;

    <Kitties<T>>::insert(kitty_id, kitty);

    <KittiesOwned<T>>::try_mutate(to, |vec| {
        vec.try_push(*kitty_id)
    }).map_err(|_| <Error<T>>::ExceedMaxKittyOwned)?;

    Ok(())
}
```

请注意我们在本教程开始时导入的 `#[transactional]` 的使用。它允许我们编写具有事务特性的函数，即只有当函数返回Ok时，才会提交对存储的更改。否则，所有更改都将被丢弃。

### **Buy a Kitty**

在我们预先用户购买 Kitty 之前，需要先确保两件事：

- 检查 Kitty 时在售的
- 检查当前 Kitty 的价格是否在用户预算范围内，用户是否有足够的余额。

替换 ACTION #6，检查是否有猫咪出售：

```rust
// Check the kitty is for sale and the kitty ask price <= bid_price
if let Some(ask_price) = kitty.price {
    ensure!(ask_price <= bid_price, <Error<T>>::KittyBidPriceTooLow);
} else {
    Err(<Error<T>>::KittyNotForSale)?;
}

// Check the buyer has enough free balance
ensure!(T::Currency::free_balance(&buyer) >= bid_price, <Error<T>>::NotEnoughBalance);
```

我们还必须验证用户是否有能力接收一只 Kitty。请记住，我们使用的 `BoundevVec` 只能容纳固定数量的 Kitty，这是在 pallet 的 `MaxKittyOwned` 常量中定义的。将 ACTION #7 替换为：

```rust
// Verify the buyer has the capacity to receive one more kitty
let to_owned = <KittiesOwned<T>>::get(&buyer);
ensure!((to_owned.len() as u32) < T::MaxKittyOwned::get(), <Error<T>>::ExceedMaxKittyOwned);

let seller = kitty.owner.clone();
```

我们将使用 [FRAME's Currency trait](https://docs.substrate.io/rustdocs/latest/frame_support/traits/tokens/currency/index.html) 的 `[transfer` method](https://crates.parity.io/frame_support/traits/tokens/currency/trait.Currency.html#tymethod.transfer) 来调整余额。了解如何使用和为什么特别使用 `transfer` method 是很重要的：

- 我们使用他的原因是为了确保我们的运行时对与其交互的所有 pallet 中的货币有相同的理解。我们确保这一点的方法是使用 `frame_support` 给我们的 `Currency trait`。
- 方便的是，它接受 `Balance` 类型，使其与我们为 `kitty.price` 创建的 `BalanceOf` 类型兼容。看看我们将使用的 transfer function 是如何构造的:
    
    ```rust
    fn transfer(
            source: &AccountId,
            dest: &AccountId,
            value: Self::Balance,
            existence_requirement: ExistenceRequirement
        ) -> DispatchResult
    ```
    

现在我们可以利用在 pallet's `Config` trait 配置的 `Currency` type  和我们在第一部分中最初开始就存在的 `ExistenceRequirement` 。
更新调用方和接收方余额的功能，替换 ACTION #8：

```rust
// Transfer the amount from buyer to seller
T::Currency::transfer(&buyer, &seller, bid_price, ExistenceRequirement::KeepAlive)?;

// Transfer the kitty from seller to buyer
Self::transfer_kitty_to(&kitty_id, &buyer)?;

// Deposit relevant Event
Self::deposit_event(Event::Bought(buyer, seller, kitty_id, bid_price))
```

<aside>
💡  `T::Currency::transfer()` 和 `Self::transfer_kitty_to()` 都可能失败，这就是为什么我们每个语句后面都跟了 `?` 的原因。如果 Err 被返回，我们也立即从函数返回。为了使储存和这些潜在的变化保持一致，我们还将这个函数标注为 `#[transactional]`。

</aside>

### **Breed Kitties**

两只 Kitties 繁殖的逻辑使使用它们的 DNA 生成一段新的 DNA。然后在 mint 一只新的 Kitty 时会用到这段 DNA。本节的模板文件中已经为您提供了这个助手函数。

复制以下代码已完成 `breed_kitty` 函数, 替换 ACTION #9：

```rust
let new_dna = Self::breed_dna(&parent1, &parent2)?;
```

不要忘记了添加 `breed_dna(&parent1, &parent2)` helper function。（代码可以在 [the helper file](https://github.com/substrate-developer-hub/substrate-docs/blob/main/static/assets/tutorials/kitties-tutorial/04-interacting-functions.rs#L227) 查看）

现在，我们已经使用了用户输入的 Kitty_ID，并将它们组合起来创建一个新的唯一 Kitty_ID，我们可以使用 mint 函数将新的 Kitty 写入 storage。替换 ACTION #10 已完成 breed_kitty extrinsic：

```rust
Self::mint(&sender, Some(new_dna), None)?;
```

### **Genesis configuration**

在我们的 pallet 准备使用的最后一步是设置我们 storage items 的 genesis 状态。我们将使用 FRAME 的 `#[pallet::genesis_config]` 来实现这一点。本质上，这允许我们声明 storage 包含在 genesis 块（创世块）中的内容。

复制以下代码到 ACTION #11：

```rust
// Our pallet's genesis configuration.
#[pallet::genesis_config]
pub struct GenesisConfig<T: Config> {
    pub kitties: Vec<(T::AccountId, [u8; 16], Gender)>,
}

// Required to implement default for GenesisConfig.
#[cfg(feature = "std")]
impl<T: Config> Default for GenesisConfig<T> {
    fn default() -> GenesisConfig<T> {
        GenesisConfig { kitties: vec![] }
    }
}

#[pallet::genesis_build]
impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
    fn build(&self) {
        // When building a kitty from genesis config, we require the dna and gender to be supplied.
        for (acct, dna, gender) in &self.kitties {
            let _ = <Pallet<T>>::mint(acct, Some(dna.clone()), Some(gender.clone()));
        }
    }
}
```

为了让我们的链知道 pallet 的 genesis configuration，我们需要修改 `node/chain_spec.rs` 文件。务必确保在 `runtime/src/lib.rs` 使用了 pallet instance name，在我们的例子中是 `SubstrateKitties` 。前往 `node/src/chain_spec.rs`， 在顶部添加 `use node_kitties_runtime::SubstrateKittiesConfig;` 并且在 `testnet_genesis` 函数中添加以下代码片段：

```rust
//-- snip --
substrate_kitties: SubstrateKittiesConfig {
    kitties: vec![],
},
//-- snip --
```

<aside>
⚠️ pallet instance name 实际叫什么，取决于你在 runtime 注册 pallet 时为它取了什么样的名字，而 `node_kitties_runtime` 是 runtime 的 包名，取决于你在 cargo.toml 中为他取了什么样的名字。

</aside>

### Build, run and interact with your Kitties

如果您已经完成了本教程前面的所有部分和步骤，您就可以运行您的链，并开始与 Kitties pallet 的所有新功能进行交互了！

使用以下命令构建和运行链：

```rust
cargo build --release
./target/release/node-kitties --dev --tmp
```

现在我们使用 Polkadot-JS Apps 检查你的工作， 就像我们之前做的。一旦您的链连接到 Polkadot-JS APP，请执行以下检查：

- Fund multiple users with tokens so they can all participate
- 让每个用户创建多个小猫
- 尝试在各种用户之间 transfer 小猫
- 尝试用各种用户为小猫设置价格
- ...

完成所有这些操作后，确认所有用户都有正确数量的小猫；Kitty 总数是正确的；并且任何其他存储变量都被正确表示。

🎉Congratulations!🎉 您已经成功创建了全功能的 Substrate chain 后端。我们的 Kitties App 的基本功能也可以抽象成其他类似的 NFT 用例。重要的是，您应该已经具备了开始创建自己的 pallet 和 dispatchable function 所需的知识。

## 下一步

转到第二部分，将您的链连接到前端，并创建一个用户界面来可视化和与您的小猫互动！

# 名词解释

**FRAME** - **Framework for Runtime Aggregation of Modularized Entities**