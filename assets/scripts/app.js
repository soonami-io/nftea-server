const { createApp, ref } = Vue;

// const { contractAbi } = PaymentWrapper.json
const primary_ingredients = [
  {
    id: 0,
    name: "Black Tea",
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
  },
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
  },
];

const contributionAmount = [1, 3, 5, 7, 10, 15, 30, 50, 75, 100, 200, 300, 500, 750, 1000];

const app_component = {
  data() {
    return {
      primary_set: primary_ingredients,
      secondary_set: secondary_ingredients,
      contribution_amount: contributionAmount,
      nft_primary_combination: [],
      nft_secondary_combination: [],
      random_primary_index: 0,
      wallet_address: null,
      web3_connected: false,
      payment_contract_address: "0x874908964FA2fF017947B880E2C9fef95bb98F66",
      mquark_contract_address: "0x50Fbd77919F74777967fEFB45a7Edad0aD5025C1",
      payment_abi: abi,
      _mquark_abi: mquark_abi,
      contribution: 0,
      show_popup: false,
      brewed_tea: null,
      backend_response: { signer: "", signature: "", uri: "", salt: "", projectId: "1", templateId: "1", collectionId: "4" },
      transactionLoading: false,
      transactionSuccess: false,
      mintedNftCount: 0,
      totalContributedAmount: 0
    };
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
    },
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
        // if (networkId == 137) { // Polygon mainnet networkid
        if (networkId == 80001) {
          // mumbai networkid
          console.log(networkId);
          try {
            const ok = await ethereum.request({ method: "eth_requestAccounts" });
            this.wallet_address = ok[0];
            this.web3_connected = true;
            localStorage.setItem("wallet_address", this.wallet_address);
          } catch {
            console.log("Err Code: ", err.code, "\nErr Message: ", err.message);
          }
        } else {
          console.log("wrong network");
          // alert("Please set network to Polygon Mainnet");
          try {
            // check if the chain to connect to is installed
            await window.ethereum.request({
              method: "wallet_switchEthereumChain",
              // params: [{ chainId: '0x89' }], // chainId must be in hexadecimal numbers
              params: [{ chainId: "0x13881" }], // chainId must be in hexadecimal numbers
            });
            const ok = await ethereum.request({ method: "eth_requestAccounts" });
            this.wallet_address = ok[0];
            this.web3_connected = true;
            localStorage.setItem("wallet_address", this.wallet_address);
          } catch (error) {
            // This error code indicates that the chain has not been added to MetaMask
            // if it is not, then install it into the user MetaMask
            if (error.code === 4902) {
              try {
                await window.ethereum.request({
                  method: "wallet_addEthereumChain",
                  params: [
                    {
                      chainId: "0x89",
                      chainName: "Polygon Mainnet",
                      rpcUrls: ["https://polygon-rpc.com"],
                      nativeCurrency: {
                        name: "MATIC",
                        symbol: "MATIC", // 2-6 characters long
                        decimals: 18,
                      },
                      blockExplorerUrls: ["https://www.polygonscan.com"],
                    },
                  ],
                });
                await window.ethereum.request({
                  method: "wallet_switchEthereumChain",
                  // params: [{ chainId: '0x89' }], // chainId must be in hexadecimal numbers
                  params: [{ chainId: "0x13881" }], // chainId must be in hexadecimal numbers
                });
                try {
                  const ok = await ethereum.request({ method: "eth_requestAccounts" });
                  this.wallet_address = ok[0];
                  this.web3_connected = true;
                  localStorage.setItem("wallet_address", this.wallet_address);
                } catch {
                  console.log("Err Code: ", err.code, "\nErr Message: ", err.message);
                }
              } catch (addError) {
                console.error("1", addError);
              }
            }
            console.error("2", error);
          }
        }
      } else {
        alert("Please install MetaMask");
        this.web3_connected = false;
        this.wallet_address = null;
        localStorage.removeItem("wallet_address");
      }
    },
    set_transaction_loading: function () {
      this.transactionLoading = !this.transactionLoading;
    },
    set_transaction_success: function () {
      this.show_popup = true;
      this.transactionLoading = !this.transactionLoading;
      this.transactionSuccess = !this.transactionSuccess;
    },
    //call backend here
    brew_tea: async function () {
      let initialValue = "";
      let result = this.nft_combination.reduce((accumulator, currentValue) => accumulator.concat(currentValue.name), initialValue);
      console.log(result.replace(/\s/g, ""));
      //call backend get the response
      this.backend_response.signer = "0xC52d3ECB7F84A27c68541933FDd5b74b96334c05";
      this.backend_response.signature ="0x"+"0f2a92500c9ffd53ddfd52f1b34de8f2abad64fbee12220d9499200b2d2b62fd0c721d43c09b0e27db3ad1c3b7f6e5def6dc401d09b0fa15e69feec2711e37001b";
      this.backend_response.uri = "https://mquark.infura-ipfs.io/ipfs/QmaPvrGQWjNQNSmPt5bbK7bGMorHooxhAXZZqRMnmSmssN";
      this.backend_response.salt = "0x21";

      this.brewed_tea = "./assets/image/dalle.png";
    },
    //show contribution popup
    set_show_popup: async function () {
      this.show_popup = !this.show_popup;
    },
    // send the transaction
    mint_with_contributing: async function () {
      const provider = new ethers.providers.Web3Provider(window.ethereum);
      const signer = provider.getSigner();
      const payment_contract = new ethers.Contract(this.payment_contract_address, this.payment_abi, signer);
      try {
        if (this.contribution < 0) {
          window.alert("The contribution can't be a minus value, or you can mint for free on the left button.");
          return;
        }
        let tx = await payment_contract.voluntaryContributionMint(
          this.backend_response.signer,
          this.backend_response.projectId,
          this.backend_response.templateId,
          this.backend_response.collectionId,
          this.backend_response.signature,
          this.backend_response.uri,
          this.backend_response.salt,
          { value: ethers.utils.parseEther(this.contribution) }
        );
        this.contribution = 0;
        this.set_transaction_loading();
        await tx.wait();
        this.set_transaction_success();
        this.mounted();
      } catch (error) {
        if (error.message == "MetaMask Tx Signature: User denied transaction signature.")  console.log(error.message);
        else window.alert("An error happened. Please check your balance is enough for the contribution or you may have already minted one!");
      }
    },
    // mint_free: async function () {
    //   console.log("send without contribution");
    //   const provider = new ethers.providers.Web3Provider(window.ethereum);
    //   const signer = provider.getSigner();
    //   const mquark_contract = new ethers.Contract(this.mquark_contract_address, this._mquark_abi, signer);
    //   try {
    //     let tx = await mquark_contract.mintFreeWithPreURI(
    //       this.backend_response.signer,
    //       this.backend_response.projectId,
    //       this.backend_response.templateId,
    //       this.backend_response.collectionId,
    //       this.backend_response.signature,
    //       this.backend_response.uri,
    //       this.backend_response.salt
    //     );
    //     this.set_transaction_loading();
    //     await tx.wait()
    //     this.set_transaction_success()
    //   } catch (error) {
    //     if (error.message == "MetaMask Tx Signature: User denied transaction signature.")  console.log(error.message);
    //     else window.alert("An error happened sending the transaction. Please check your balance is enough for the contribution.");
       
    //   }
    // },
    setContribution: function (amount) {
      this.contribution = amount;
      console.log(amount);
    },
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
  mounted: async function () {
    const provider = new ethers.providers.JsonRpcProvider("https://polygon-mumbai.g.alchemy.com/v2/6C2ub0-l9nAs7qb_17wH8OVfs0ECYfGC");
    const mquark_contract = new ethers.Contract(this.mquark_contract_address, this._mquark_abi, provider);
    const payment_contract = new ethers.Contract(this.payment_contract_address, this.payment_abi, provider);

    //! update here after deploying to mainnet
    let result = await mquark_contract.getProjectCollection("1", "1", "4");
    this.mintedNftCount = result.mintCount.toString();

    let _totalContributedAmount = await payment_contract.getTotalContribution();
    this.totalContributedAmount = ethers.utils.formatEther(_totalContributedAmount);
    
  },
};

createApp(app_component).mount("#vue-app");

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
    `;
