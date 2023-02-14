const { createApp, ref } = Vue;
const backendResponse = {
  "signature": "ab23f9db62245bf5fbdac90ffee00b2bed8895b3dd34c15b7936f7baee2a6e94372418fd34c26810f0d2f81e5b4982078f7348f73b0960daacbe3971b49c1a121b",
  "ipfs_uri": "ipfs://QmYFyX7vrt6zpGfiGsoJnTx3SztYkeKsHSqRDix9YMMmgM",
  "metadata": {
      "name": "NFTea TESTNET",
      "image": "https://ipfs.io/ipfs/QmcQrUhV9wk24PUXC72gJL1JnSvshBRZZ4E2EJYJ8643V8/51.png",
      "description": "Our NFTeas are truly special blend utilising the power of mQuark , they are more than an image,  they are transformed into living, breathing pieces of art, each with its own unique flavour and personality. Infinitely upgradable through this metadata they offer true interoperability - take them anywhere!",
      "origins": {
          "template": {
              "id": "1",
              "name": "mQuark Beverage",
              "image": "https://ipfs.io/ipfs/QmTxpSbXqB5m7PsnEzofMnVTCoyUCJy1i224t2Kv9WZoa4",
              "description": "This is a Beverage Template, a simple representation of beverage-typed NFT collections.",
              "attributes": [
                  {
                      "value": "Type"
                  },
                  {
                      "value": "Temperature"
                  },
                  {
                      "value": "Ingredient Type"
                  },
                  {
                      "value": "Sweetness Level"
                  },
                  {
                      "value": "Size"
                  },
                  {
                      "value": "Milk Type"
                  },
                  {
                      "value": "Effect"
                  },
                  {
                      "value": "Container"
                  },
                  {
                      "value": "Rarity"
                  }
              ]
          },
          "project": {
              "id": "1",
              "name": "Flying Fish Tea Co.",
              "image": "https://cdn.shopify.com/s/files/1/0531/1116/0993/files/green_logo-2-2-2-2-2_140x.jpg?v=1636920599",
              "description": "https://www.flyingfishtea.co.uk/"
          },
          "collection": {
              "id": "1",
              "name": "NFTea",
              "description": "Once upon a time, in a land where teas were kings, six unique ones lived together in harmony. Black tea, White tea, Green tea, Rooibos tea, Pu-erh tea, and Oolong tea each had their own special qualities and lived in separate tea gardens, content with their daily routines. But one day, they heard whispers of a revolutionary new world, a place where they could become more than just tea - the world of Web3.\nExcited by the prospect of becoming something truly unique, the teas decided to embark on a journey together to discover this magical land. Along the way, they gathered special ingredients to enhance their flavors and make themselves stand out from the rest.\nFinally, they arrived at the entrance to the Web3 world - a sprawling marketplace filled with opportunities and challenges. As they explored this new and exciting place, they discovered that they could use blockchain technology to create unique digital representations of themselves, each with their own special blend of ingredients.\nThe teas worked tirelessly, perfecting their digital creations and blending themselves with the finest ingredients. And soon, they were transformed into living, breathing pieces of art, each with its own unique flavor and personality.\nAs they explored the Web3 world, they encountered other digital creations and formed friendships with them. They learned that they could trade their digital representations with others and that their creations would live forever, becoming a part of Web3's rich history.\nAnd so, the six teas lived happily ever after, continuing to explore the wonders of web3 and sharing their unique creations with the world. They knew that they would never forget their journey and the lessons they had learned along the way.",
              "image": "ipfs://[collection-cid]",
              "variations": "dynamic",
              "attributes": [
                  {
                      "trait_type": "ingredient",
                      "value": "blacktea"
                  },
                  {
                      "trait_type": "ingredient",
                      "value": "whitetea"
                  },
                  {
                      "trait_type": "ingredient",
                      "value": "jasmine"
                  }
              ]
          }
      },
      "attributes": [
          {
              "trait_type": "ingredient",
              "value": "blacktea"
          },
          {
              "trait_type": "ingredient",
              "value": "whitetea"
          },
          {
              "trait_type": "ingredient",
              "value": "jasmine"
          }
      ]
  }
}
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
      //!change addresses
      // payment_contract_address: "0x874908964FA2fF017947B880E2C9fef95bb98F66",
      payment_contract_address: "0xDEE416e75A8443dE13DbbBf0C3b558F7fC45eC2c",
      // mquark_contract_address: "0x50Fbd77919F74777967fEFB45a7Edad0aD5025C1",
      mquark_contract_address: "0x30aB861483079Cbb58B0DC95804B861DD5Aa631F",
      payment_abi: abi,
      _mquark_abi: mquark_abi,
      contribution: 0,
      show_popup: false,
      brewed_tea: null,
      //!change below
      backend_response: { signer: "0x49dbfb94314CF76b2Fe990e9dc5E59AF7b68E4b1", signature: "", uri: "", salt: "0x01", projectId: "1", templateId: "1", collectionId: "1",isLoading:false },
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
        if (networkId == 137) { // Polygon mainnet networkid
        // if (networkId == 80001) {
          // mumbai networkid
          console.log("networkid", networkId);
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
              params: [{ chainId: '0x89' }], // chainId must be in hexadecimal numbers
              // params: [{ chainId: "0x13881" }], // chainId must be in hexadecimal numbers
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
                  params: [{ chainId: '0x89' }], // chainId must be in hexadecimal numbers
                  // params: [{ chainId: "0x13881" }], // chainId must be in hexadecimal numbers
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
      if(this.nft_combination == "") {
        window.alert("Please select at least a one ingredient!")
        return;
      }
      let _combination = this.nft_combination.reduce((accumulator, currentValue) => accumulator.concat(currentValue.name+"_"), initialValue);
      _combination = _combination.substring(0, _combination.length - 1);

      const data = {
        combination: _combination
      };
      this.backend_response.isLoading= true;
      try {
        const response = await axios.post('https://3.70.32.6/uri', data,
        {
          'Access-Control-Allow-Origin': 'https://3.70.32.6/uri'
        });
        let _respone = response.data;

        window.localStorage.setItem("brewed_tea",JSON.stringify(_respone))

        this.backend_response.signature = `0x${_respone.signature}`
        let cid = _respone.metadata.image.split("/");
        cid = cid[4].concat("/"+cid[5]);
        this.brewed_tea = `https://teal-worthwhile-mandrill-830.mypinata.cloud/ipfs/${cid}`
        this.backend_response.salt = "0x01";
        this.backend_response.uri = _respone.ipfs_uri;
        this.backend_response.isLoading= false;
      } catch(error) {
        console.error("error",error);
        if (error.response.data === "NftTaken") alert("NFT is taken. Please reorder your ingridients, or change them or add some more ingridents.")
        this.backend_response.isLoading= false;
      }
      this.backend_response.isLoading= false;
    },
    //show contribution popup
    set_show_popup: async function () {
      this.show_popup = !this.show_popup;
    },
    // send the transaction
    mint_with_contributing: async function () {
      const provider = new ethers.providers.Web3Provider(window.ethereum);
      const signer = await provider.getSigner();
      const signer_address = await signer.getAddress();
      const payment_contract = new ethers.Contract(this.payment_contract_address, this.payment_abi, signer);

      try {
        const mintStatus = await payment_contract.mintedAddresses(signer_address);
        if(mintStatus) window.alert("You have already minted NFTea!");
      } catch(error) {
        console.error("minting contract call reverted!");
        console.error(error);
      }

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
        else window.alert("An error happened. You may have forgotten to input a MATIC value. Or please check if your balance has sufficient fund for the contribution. Else you may have already minted one NFTea!");
      }
    },
    mint_free: async function () {
      const provider = new ethers.providers.Web3Provider(window.ethereum);
      const signer = await provider.getSigner();
      const signer_address = await signer.getAddress();
      const payment_contract = new ethers.Contract(this.payment_contract_address, this.payment_abi, signer);
      try {
        const mintStatus = await payment_contract.mintedAddresses(signer_address);
        if(mintStatus) window.alert("You have already minted NFTea!");
      } catch(error) {
        console.error("minting contract call reverted!");
        console.error(error);
      }

      try {
        let tx = await payment_contract.voluntaryContributionMint(
          this.backend_response.signer,
          this.backend_response.projectId,
          this.backend_response.templateId,
          this.backend_response.collectionId,
          this.backend_response.signature,
          this.backend_response.uri,
          this.backend_response.salt,
          { value: 0 }
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
    setContribution: function (amount) {
      this.contribution = amount;
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
    if(window.localStorage.getItem("brewed_tea")) this.backend_response.isLoading = true;
    const provider = new ethers.providers.JsonRpcProvider("https://polygon-mainnet.g.alchemy.com/v2/N9HCtvuidF3dTQuRuQ3ORN304LADukdp");
    const mquark_contract = new ethers.Contract(this.mquark_contract_address, this._mquark_abi, provider);
    const payment_contract = new ethers.Contract(this.payment_contract_address, this.payment_abi, provider);

    let result = await mquark_contract.getProjectCollection("1", "1", "1");
    this.mintedNftCount = result.mintCount.toString();

    let _totalContributedAmount = await payment_contract.getTotalContribution();
    this.totalContributedAmount = ethers.utils.formatEther(_totalContributedAmount);
    if(window.localStorage.getItem("brewed_tea")){

      let _response = JSON.parse(window.localStorage.getItem("brewed_tea"));
      this.backend_response.signature = `0x${_response.signature}`
      let cid = _response.metadata.image.split("/");
      cid = cid[4].concat("/"+cid[5]);
      this.brewed_tea = `https://teal-worthwhile-mandrill-830.mypinata.cloud/ipfs/${cid}`
      this.backend_response.salt = "0x01";
      this.backend_response.uri = _response.ipfs_uri;
    } 
    this.backend_response.isLoading = false;
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
