const { createApp } = Vue;
const primary_ingredients = [
    {
        id: 0,
        name: "Back Tea",
        image: "",
        weight: 2,
    },
    {
        id: 1,
        name: "White Tea",
        image: "",
        weight: 2,
    },
    {
        id: 2,
        name: "Green Tea",
        image: "",
        weight: 2,
    },
    {
        id: 3,
        name: "Rooibos Tea",
        image: "",
        weight: 2,
    },
    {
        id: 4,
        name: "Oolong Tea",
        image: "",
        weight: 2,
    },
    {
        id: 5,
        name: "Pu-erh Tea",
        image: "",
        weight: 2,
    }
];

const secondary_ingredients = [
    {
        id: 6,
        name: "Cinnamon",
        image: "",
        weight: 1,
    },
    {
        id: 7,
        name: "Bergamot",
        image: "",
        weight: 1,
    },
    {
        id: 8,
        name: "Jasmine",
        image: "",
        weight: 1,
    },
    {
        id: 9,
        name: "Star Anise",
        image: "",
        weight: 1,
    },
    {
        id: 10,
        name: "Mixed berries",
        image: "",
        weight: 1,
    },
    {
        id: 11,
        name: "Mixed fruit",
        image: "",
        weight: 1,
    },
    {
        id: 12,
        name: "Pineapple",
        image: "",
        weight: 1,
    },
    {
        id: 13,
        name: "Mint",
        image: "",
        weight: 1,
    },
    {
        id: 14,
        name: "Citrus zest",
        image: "",
        weight: 1,
    },
    {
        id: 15,
        name: "Chamomile",
        image: "",
        weight: 1,
    },
    {
        id: 16,
        name: "Lemongrass",
        image: "",
        weight: 1,
    },
    {
        id: 17,
        name: "Hibiscus",
        image: "",
        weight: 1,
    },
    {
        id: 18,
        name: "Ginger",
        image: "",
        weight: 1,
    },
    {
        id: 19,
        name: "Cardamom",
        image: "",
        weight: 1,
    }
];

const app_component = {
    data() {
        return {
            primary_set: primary_ingredients,
            secondary_set: secondary_ingredients,
            random_primary_index: 0,
            wallet_address: null,
            web3_connected: false,
            contract_address: "0x0",
            abi: [],
        }
    },
    methods: {
        randomize_primary: function() {
            this.random_primary_index = Math.floor(Math.random() * this.primary_set.length);
        },
        connect_wallet: async function() {
            if (typeof window.ethereum !== "undefined") {
                try {
                    const ok = await ethereum.request({method: "eth_requestAccounts"})
                    this.wallet_address = ok[0];
                    this.web3_connected = true;
                    localStorage.setItem("wallet_address", this.wallet_address);
                } catch {
                    console.log("Err Code: ", err.code, "\nErr Message: ", err.message)
                }               
            } else {
                alert("Please install MetaMast");
                this.web3_connected = false;
                this.wallet_address = null;
                localStorage.removeItem("wallet_address");
            }
        },
        execute_web3: async function() {
            console.log("some web3 getting executed!")
            const provider = new ethers.providers.Web3Provider(window.ehtereum);
            const signer = provider.getSigner();
            const contract = new ethers.Contract(
                this.contract_address,
                this.abi,
                signer,
            );
            try {
                // await contract.[function](params); calling the mint function on the contract
            } catch(error) {
                console.log(error);
            }
        }
    },
    template: `
        <div id="app-header">
            <p>{{ primary_set[random_primary_index].name }}</p>
            <button @click="randomize_primary" class="button"> Get Primary </button>
            <button @click="connect_wallet" class="button"> Connect Wallet </button>
        </div>
    `
}

createApp(app_component).mount('#app');

