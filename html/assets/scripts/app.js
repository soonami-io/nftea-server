const { createApp, ref } = Vue;
// const { contractAbi } = PaymentWrapper.json
const primary_ingredients = [
    {
        id: 0,
        name: "Back Tea",
        image: "assets/image/ingridients/black_tea.png",
        weight: 2,
    },
    {
        id: 1,
        name: "White Tea",
        image: "assets/image/ingridients/white_tea.png",
        weight: 2,
    },
    {
        id: 2,
        name: "Green Tea",
        image: "assets/image/ingridients/green_tea.png",
        weight: 2,
    },
    {
        id: 3,
        name: "Rooibos Tea",
        image: "assets/image/ingridients/Rooibos.png",
        weight: 2,
    },
    {
        id: 4,
        name: "Oolong Tea",
        image: "assets/image/ingridients/Oolong.png",
        weight: 2,
    },
    {
        id: 5,
        name: "Pu-erh Tea",
        image: "assets/image/ingridients/Pu-erh_tea_leave.png",
        weight: 2,
    }
];

const secondary_ingredients = [
    {
        id: 6,
        name: "Cinnamon",
        image: "assets/image/ingridients/Cinnamon.png",
        weight: 1,
    },
    {
        id: 7,
        name: "Bergamot",
        image: "assets/image/ingridients/bergamot_leaves.png",
        weight: 1,
    },
    {
        id: 8,
        name: "Jasmine",
        image: "assets/image/ingridients/jasmine.png",
        weight: 1,
    },
    {
        id: 9,
        name: "Star Anise",
        image: "assets/image/ingridients/star_anise.png",
        weight: 1,
    },
    {
        id: 10,
        name: "Mixed berries",
        image: "assets/image/ingridients/Mixed_berries.png",
        weight: 1,
    },
    {
        id: 11,
        name: "Mixed fruit",
        image: "assets/image/ingridients/Mixed_fruits.png",
        weight: 1,
    },
    {
        id: 12,
        name: "Pineapple",
        image: "assets/image/ingridients/pineapple.png",
        weight: 1,
    },
    {
        id: 13,
        name: "Mint",
        image: "assets/image/ingridients/Mint.png",
        weight: 1,
    },
    {
        id: 14,
        name: "Citrus zest",
        image: "assets/image/ingridients/Citrus_Zest.png",
        weight: 1,
    },
    {
        id: 15,
        name: "Chamomile",
        image: "assets/image/ingridients/chamomile.png",
        weight: 1,
    },
    {
        id: 16,
        name: "Lemongrass",
        image: "assets/image/ingridients/Lemongrass.png",
        weight: 1,
    },
    {
        id: 17,
        name: "Hibiscus",
        image: "assets/image/ingridients/hibiscus_leaves.png",
        weight: 1,
    },
    {
        id: 18,
        name: "Ginger",
        image: "assets/image/ingridients/ginger.png",
        weight: 1,
    },
    {
        id: 19,
        name: "Cardamom",
        image: "assets/image/ingridients/Cardamom.png",
        weight: 1,
    }
];

const app_component = {
    data() {
        return {
            primary_set: primary_ingredients,
            secondary_set: secondary_ingredients,
            nft_primary_combination: [],
            nft_secondary_combination: [],
            random_primary_index: 0,
            wallet_address: null,
            web3_connected: false,
            payment_contract_address: "0x12CB33d84E119EE06Ad2BcDea0bb269E04cf373e",
            mquark_contract_address: "0x50Fbd77919F74777967fEFB45a7Edad0aD5025C1",
            payment_abi: abi,
            _mquark_abi: mquark_abi
        }
    },
    computed: {
        nft_primary_combination_count() {
            return this.nft_primary_combination.length;
        },
        nft_secondary_combination_count() {
            return this.nft_secondary_combination.length;
        },
        nft_combination() {
            return [...this.nft_primary_combination, ...this.nft_secondary_combination];
        }
    },
    methods: {
        randomize_primary: function () {
            this.random_primary_index = Math.floor(Math.random() * this.primary_set.length);
        },
        connect_wallet: async function () {

            if (typeof window.ethereum !== "undefined") {
                const networkId = await window.ethereum.request({
                    method: "net_version",
                });
                if (networkId == 137) { // Polygon mainnet networkid
                    console.log(networkId)
                    try {
                        const ok = await ethereum.request({ method: "eth_requestAccounts" })
                        this.wallet_address = ok[0];
                        this.web3_connected = true;
                        localStorage.setItem("wallet_address", this.wallet_address);
                    } catch {
                        console.log("Err Code: ", err.code, "\nErr Message: ", err.message)
                    }
                } else {
                    alert("Please set network to Polygon Mainnet");
                    try {
                        // check if the chain to connect to is installed
                        await window.ethereum.request({
                            method: 'wallet_switchEthereumChain',
                            params: [{ chainId: '0x89' }], // chainId must be in hexadecimal numbers
                        });
                    } catch (error) {
                        // This error code indicates that the chain has not been added to MetaMask
                        // if it is not, then install it into the user MetaMask
                        if (error.code === 4902) {
                            try {
                                await window.ethereum.request({
                                    method: 'wallet_addEthereumChain',
                                    params: [
                                        {
                                            chainId: '0x89',
                                            rpcUrl: "https://polygon-rpc.com",
                                        },
                                    ],
                                });
                            } catch (addError) {
                                console.error(addError);
                            }
                        }
                        console.error(error);
                    }
                }

            } else {
                alert("Please install MetaMast");
                this.web3_connected = false;
                this.wallet_address = null;
                localStorage.removeItem("wallet_address");
            }
        },
        //most probably;
        //template ID will be : 1
        //collection ID will be : 1
        //and project ID will be : 1
        //because 99%, NFTea will be the first project's first collection that inherits the first template :)
        execute_web3: async function () {
            // console.log("test");
            // console.log("some web3 getting executed!")
            const provider = new ethers.providers.Web3Provider(window.ethereum);

            const signer = provider.getSigner();

            //this.callBackend(signer);

            const payment_contract = new ethers.Contract(
                this.payment_contract_address,
                this.payment_abi,
                signer,
            );
            const mquark_contract = new ethers.Contract(
                this.mquark_contract_address,
                this._mquark_abi,
                signer,
            );

            //if the user would like to donate call payment function

        },
        callBackend: async function (signer) {
            let initialValue = "";
            let result = this.nft_combination.reduce((accumulator, currentValue) => accumulator.concat(currentValue.name), initialValue)
            console.log(result.replace(/\s/g, ""));
        }
    },
    watch: {
        nft_primary_combination_count(value) {
            if (value > 2) {
                this.nft_primary_combination.pop();
                alert("Select only two (2) primary ingridents.");
            }
        },
        nft_secondary_combination_count(value) {
            if (value > 5) {
                this.nft_secondary_combination.pop();
                alert("Select only five (5) primary ingridents.");
            }
        },
    },
    mounted() {
        console.log("heelo from mounted lifecycle!");
        // add eventlistenr here
    }
}

createApp(app_component).mount('#vue-app');

const template = `
        <div id="app-header">
            <p>{{ primary_set[random_primary_index].name }}</p>
            <button @click="randomize_primary" class="button"> Get Primary </button>
            <button @click="connect_wallet" class="button"> Connect Wallet </button>
            <form>      
                <fieldset>      
                    <legend>Pick 2 of your primary ingredients</legend>
                    <div class="primary form_item">
                        <label class="checkbox_item" v-for="ingridient in primary_set" :key="ingridient.id">
                            <input type="checkbox" name="primary_ingrident" :value="ingridient.id" v-model="ingrident.selected">
                            {{ ingridient.name }}
                        </label>
                        <br>
                    </div>    
                </fieldset>
            </form>
        </div>
    `