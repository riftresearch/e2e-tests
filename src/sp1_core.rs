use alloy::hex;

pub const SP1_CIRCUIT_VERIFICATION_HASH: [u8; 32] = hex!("007bcdb7ee0e28808292c85204587c49ac3ced1ebc6d23e73632b50ad380795f");
// sp1 V2_0_0_SP1_VERIFIER_PLONK bytecode
pub const SP1_VERIFIER_BYTECODE: &str = "608060405234801561001057600080fd5b50600436106100575760003560e01c80632a5104361461005c57806341493c60146100915780636b61d8e7146100a65780637e4f7a8a146100b9578063ffa1ad74146100dc575b600080fd5b7f4aca240a3e5296e6a565f98dc728c6f48f8de4792a8fa365038c3b86952176f55b6040519081526020015b60405180910390f35b6100a461009f366004612314565b610104565b005b61007e6100b436600461238e565b6102a0565b6100cc6100c73660046123d0565b610307565b6040519015158152602001610088565b6040805180820182526006815265076322e302e360d41b60208201529051610088919061246a565b600061011360048284866124b8565b61011c916124e2565b90507f4aca240a3e5296e6a565f98dc728c6f48f8de4792a8fa365038c3b86952176f5632565120560e11b6001600160e01b03198316146101885760405163988066a160e01b81526001600160e01b031980841660048301528216602482015260440160405180910390fd5b600061019487876102a0565b60408051600280825260608201835292935060009290916020830190803683370190505090508860001c816000815181106101d1576101d1612512565b6020026020010181815250508160001c816001815181106101f4576101f4612512565b6020908102919091010152600030637e4f7a8a610214886004818c6124b8565b856040518463ffffffff1660e01b815260040161023393929190612528565b602060405180830381865afa158015610250573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061027491906125a1565b905080610294576040516309bde33960e01b815260040160405180910390fd5b50505050505050505050565b60006001600160fd1b0360001b600284846040516102bf9291906125ca565b602060405180830381855afa1580156102dc573d6000803e3d6000fd5b5050506040513d601f19601f820116820180604052508101906102ff91906125da565b169392505050565b6000604051610240810161031a846105fd565b6103248585610610565b61032d8661064e565b61033687610664565b600061034386868a610781565b905061034e81610aa7565b905061035a8189610afc565b90506103668189610b79565b5060608201516000805160206126148339815191526000805160206126348339815191526103998463020000008561227e565b086101c0840152506103ac818587610bd2565b6103b782868a610e22565b91506000805160206126148339815191528183086101a0840152506103dc9050611121565b6103e586611f8e565b6103ee86611edf565b6103f786611cb5565b61040086611803565b610409866115be565b61041286611206565b610200015190506122c3565b60405162461bcd60e51b815260206004820152601d60248201527f77726f6e67206e756d626572206f66207075626c696320696e707574730000006044820152606481fd5b60405162461bcd60e51b815260206004820152601260248201527132b93937b91032b19037b832b930ba34b7b760711b6044820152606481fd5b60405162461bcd60e51b815260206004820152601860248201527f696e707574732061726520626967676572207468616e207200000000000000006044820152606481fd5b60405162461bcd60e51b815260206004820152601060248201526f77726f6e672070726f6f662073697a6560801b6044820152606481fd5b60405162461bcd60e51b815260206004820152601660248201527537b832b734b733b9903134b3b3b2b9103a3430b7103960511b6044820152606481fd5b60405162461bcd60e51b815260206004820152600d60248201526c6572726f722070616972696e6760981b6044820152606481fd5b60405162461bcd60e51b815260206004820152600c60248201526b6572726f722076657269667960a01b6044820152606481fd5b60405162461bcd60e51b81526020600482015260146024820152736572726f722072616e646f6d2067656e206b7a6760601b6044820152606481fd5b6002811461060d5761060d61041e565b50565b60005b81811015610649576000805160206126348339815191528335111561063a5761063a61049d565b60209290920191600101610613565b505050565b610360818114610660576106606104e2565b5050565b6101808101600080516020612634833981519152813511156106885761068861051a565b506101a08101600080516020612634833981519152813511156106ad576106ad61051a565b506101c08101600080516020612634833981519152813511156106d2576106d261051a565b506101e08101600080516020612634833981519152813511156106f7576106f761051a565b5061020081016000805160206126348339815191528135111561071c5761071c61051a565b506102608101600080516020612634833981519152813511156107415761074161051a565b50610300810160005b600181101561064957600080516020612634833981519152823511156107725761077261051a565b6020919091019060010161074a565b600060405161024081016467616d6d6181527f0f68498bcb8bad722bb26d7fdf86ec47e0608f5c7426553b32b6475f107d2f6f60208201527f02c32c714551e3a5ae040a4d1ab1c202a8e60ee284dc28854d5315d87cef56f860408201527f092aad8aced0070c40da5a17879021548381419825e79e4d9a7b4074625c54e860608201527f197539b4c3a63c778ce4d380f825aa86ed0fb79f2636e221e6b7686483bb29d860808201527f14e3f0f36bef38d5f4eb71501e84f1c92b9d28da48a0a83d7804b89da625507f60a08201527f2a3d669b0c1e80c3b920fdbc9cb6b244662f66a664cb9b8175113dc9a447b70760c08201527f106055b838dd0ff1ddbffc86e2d15b2d6cd4d444612f27399a3bd6b623370f5860e08201527f08996b79136f308458f3487a7854d89130c9416899fc6cd706b6e5c37066e2b36101008201527f1d26c7268a5bf0d7aad60699cb80c395e91d0d6674f857d79cc1d4ce5b53685a6101208201527f0e43522b5c7a9a565087e0b82aa5e55d4b0b3f3d98b364a6f7aa85ef907a6a806101408201527f115041e8ec2e9d42421162ab6914888c70916dc6f9980849821848c79f55df826101608201527f2f5d1c7c542b18fef0c6ee9edd0bd56116368f222b16bf40bbc71461844c255e6101808201527f1b74ec8cba7a98b1cecda2b4feb5c5f96cebbfe50be4148d0c68eeeccb0a39d26101a08201527f1fd962a4192e713de0e1520af64ee659164f0d22743d628b81ff849718977d946101c08201527f0aa3276c8202ad61d27e33a7859b2007e61222d72757b60334f020bdea1010316101e08201527f175eeaaecf70c2fe5a97bc31cd11eb7711e8bce16ba0b26c5a4cc5fe39c8dc756102008201527f2696d97ac82d31c982748ba92750ac9e5495e983bab692979efd9e37c12f420c6102208201527f2b343ec63a1fef8f5db4f767292478f1ed9aefe75c867549c356e235198c5fb1610240820152610260810160208602808883379081019060c0808784375061030501905060208282601b820160025afa905080610a8357610a8361058d565b50805192505060008051602061261483398151915282066040820152509392505050565b600060405161024060405101636265746181528360208201526020816024601c840160025afa80610ada57610ada61058d565b5080519250506000805160206126148339815191528206602082015250919050565b600060405161024060405101606564616c7068618252602082018681526020810190506103208601600160400280828437928301929190910190506040610220870182375060208282601b850160025afa905080610b5c57610b5c61058d565b505160008051602061261483398151915281069091529392505050565b60405161024060405101637a657461815283602082015260c0808401604083013760208160e4601c840160025afa80610bb457610bb461058d565b50516000805160206126148339815191529006606091909101525050565b600060405160608101516101c0820151915085610bf181878585610c4a565b60009250600091505b85821015610c4057600080516020612614833981519152853582510992506000805160206126148339815191528385086020958601959094506001929092019101610bfa565b5050509392505050565b6000805160206126148339815191527f30644e5aaf0a66b91f8030da595e7d1c6787b9b45fc54c546729acf1ff053609830960018560005b86811015610cdf576000805160206126148339815191528360008051602061261483398151915203860882526000805160206126148339815191526000805160206125f48339815191528409925060209190910190600101610c82565b50610ceb818789610d5c565b50600190508560005b86811015610d525760008051602061261483398151915283600080516020612614833981519152868551090982526020820191506000805160206126148339815191526000805160206125f483398151915284099250600101610cf4565b5050505050505050565b600183526000805b83811015610da0578185015182840151600080516020612614833981519152818309905060208401935080848801525050600181019050610d64565b506020810382019150808401935050610dcf60208401600260008051602061261483398151915203855161227e565b60005b83811015610e1b576020850394508251600080516020612614833981519152865184098452600080516020612614833981519152818409601f1990940193925050600101610dd2565b5050505050565b600060405160608101516101c082015191506103208401600080610e4c8960208501358535610f54565b9150610e60896301131e898a018787610e84565b90506000805160206126148339815191528082840987089998505050505050505050565b6000610e9f85856000805160206125f483398151915261227e565b6000805160206126148339815191528160008051602061261483398151915203840894506000805160206126148339815191527f30644e5aaf0a66b91f8030da595e7d1c6787b9b45fc54c546729acf1ff05360982099050610f22867f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593efffffff8761227e565b945060008051602061261483398151915285820990506000805160206126148339815191528482099695505050505050565b600083526000602084015280604084015250806060830152506000608082015360306081820153600060828201536042608382015360536084820153604260858201536032608682015360326087820153602d608882015360506089820153606c608a820153606f608b820153606e608c820153606b608d820153600b608e8201536000602082608f8460025afa80610fef57610fef61058d565b8251600160208501536042602185015360536022850153604260238501536032602485015360326025850153602d602685015360506027850153606c6028850153606f6029850153606e602a850153606b602b850153600b602c850153602084602d8660025afa9150816110655761106561058d565b8351186020840152600260408401536042604184015360536042840153604260438401536032604484015360326045840153602d604684015360506047840153606c6048840153606f6049840153606e604a840153606b604b840153600b604c84015360208301602081602d8360025afa915050806110e6576110e661058d565b50600080516020612614833981519152600160801b8351099050602082015160801c6000805160206126148339815191528183089392505050565b604051610240604051016101c0820151600080516020612614833981519152600160008051602061261483398151915203606085015108611183837f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593efffffff8361227e565b90506000805160206126148339815191527f30644e5aaf0a66b91f8030da595e7d1c6787b9b45fc54c546729acf1ff05360982099050600080516020612614833981519152828209845193509150600080516020612614833981519152905082820990506000805160206126148339815191528282099050806080840152505050565b60405161024081016101608201518152610180820151602082015261028083013560408201526102a08301356060820152610220830135608082015261024083013560a08201526102c083013560c08201526102e083013560e082015260608201516101008201526101e08201516101208201526020816101408360025afa80611292576112926105c1565b60008051602061261483398151915282510690508160408101925061028085013581526102a085013560208201526112d083836102c0880184612209565b61016084016112e58484610220890184612209565b61014085016112f984610260890183612250565b7f1fa4be93b5e7f7e674d5059b63554fab99638b304ed8310e9fa44c281ac9b03b85527f1a01ae7fac6228e39d3cb5a5e71fd31160f3241e79a5f48ffb3737e6c389b7216020860152805160408087019182529095908160608160075afa9150816113665761136661058d565b60208101915081517f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd4703825261139e86828586612100565b5050836040850194506113bb8560608801516102808a0184612197565b6000805160206126148339815191526000805160206125f48339815191526060880151099550600080516020612614833981519152868509935061140585856102c08a0184612209565b61141185828485612100565b50602082810180517f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd470381528251865291810151908501527f198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c260408501527f1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed60608501527f090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b60808501527f12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa60a0850152905160c0840152805160e08401527f22f1acbb03c4508760c2430af35865e7cdf9f3eb1224504fdcc3708ddb954a486101008401527f2a344fad01c2ed0ed73142ae1752429eaea515c6f3f6b941103cc21c2308e1cb6101208401527f159f15b842ba9c8449aa3268f981010d4c7142e5193473d80b464e964845c3f86101408401527f0efd30ac7b6f8d0d3ccbc2207587c2acbad1532dc0293f0d034cf8258cd428b36101608401529250610649905081604051602060006101808460085afa806115af576115af610558565b50600051610200919091015250565b6040516101e081015160e0820151610160830190815261010083015161018084015261012083015161014084015261024083019161026084019161028085019190819061160d86838a84612209565b611620826101808a016101408a01612250565b6000805160206126148339815191528383099150611643868360408b0184612209565b611656826101a08a016101408a01612250565b6000805160206126148339815191528383099150611679868360808b0184612209565b61168c826101c08a016101408a01612250565b60008051602061261483398151915283830991507f0f68498bcb8bad722bb26d7fdf86ec47e0608f5c7426553b32b6475f107d2f6f86527f02c32c714551e3a5ae040a4d1ab1c202a8e60ee284dc28854d5315d87cef56f885526116f2848388846121c2565b611705826101e08a016101408a01612250565b60008051602061261483398151915283830991507f092aad8aced0070c40da5a17879021548381419825e79e4d9a7b4074625c54e886527f197539b4c3a63c778ce4d380f825aa86ed0fb79f2636e221e6b7686483bb29d8855261176b848388846121c2565b61177e826102008a016101408a01612250565b610300880160008051602061261483398151915284840992507f2696d97ac82d31c982748ba92750ac9e5495e983bab692979efd9e37c12f420c87527f2b343ec63a1fef8f5db4f767292478f1ed9aefe75c867549c356e235198c5fb186526117e9858489856121c2565b6117f883826101408b01612250565b505050505050505050565b6040516467616d6d616102408201908152606082015161026083015260e08201516102808301526101008201516102a083015260c0836102c08401377f0f68498bcb8bad722bb26d7fdf86ec47e0608f5c7426553b32b6475f107d2f6f6101408201527f02c32c714551e3a5ae040a4d1ab1c202a8e60ee284dc28854d5315d87cef56f86101608201527f092aad8aced0070c40da5a17879021548381419825e79e4d9a7b4074625c54e8610180808301919091527f197539b4c3a63c778ce4d380f825aa86ed0fb79f2636e221e6b7686483bb29d86101a0808401919091527f2696d97ac82d31c982748ba92750ac9e5495e983bab692979efd9e37c12f420c6101c0808501919091527f2b343ec63a1fef8f5db4f767292478f1ed9aefe75c867549c356e235198c5fb16101e0808601919091526101208601516102008087019190915293870135610220860152918601356102408501528501356102608401528401356102808301528301356102a08201526102c081016103008401602081833750610260840135602091820152601b906102e5906101e085018285850160025afa92505050806119b9576119b961058d565b506101e00180516000805160206126148339815191529006905250565b604051610240604051017f106055b838dd0ff1ddbffc86e2d15b2d6cd4d444612f27399a3bd6b623370f5881527f08996b79136f308458f3487a7854d89130c9416899fc6cd706b6e5c37066e2b36020820152611a40604082016101808501358360e0860161216c565b7f1d26c7268a5bf0d7aad60699cb80c395e91d0d6674f857d79cc1d4ce5b53685a81527f0e43522b5c7a9a565087e0b82aa5e55d4b0b3f3d98b364a6f7aa85ef907a6a806020820152611aa0604082016101a08501358360e086016121c2565b6000805160206126148339815191526101a0840135610180850135097f115041e8ec2e9d42421162ab6914888c70916dc6f9980849821848c79f55df8282527f2f5d1c7c542b18fef0c6ee9edd0bd56116368f222b16bf40bbc71461844c255e6020830152611b1760408301828460e087016121c2565b507f1b74ec8cba7a98b1cecda2b4feb5c5f96cebbfe50be4148d0c68eeeccb0a39d281527f1fd962a4192e713de0e1520af64ee659164f0d22743d628b81ff849718977d946020820152611b78604082016101c08501358360e086016121c2565b7f0aa3276c8202ad61d27e33a7859b2007e61222d72757b60334f020bdea10103181527f175eeaaecf70c2fe5a97bc31cd11eb7711e8bce16ba0b26c5a4cc5fe39c8dc756020820152611bd3604082018260e0850180612100565b6103008301610320840160005b6001811015611c20578135845260208201356020850152611c0a6040850184358660e089016121c2565b6020929092019160409190910190600101611be0565b5050507f14e3f0f36bef38d5f4eb71501e84f1c92b9d28da48a0a83d7804b89da625507f81527f2a3d669b0c1e80c3b920fdbc9cb6b244662f66a664cb9b8175113dc9a447b7076020820152611c7e60408201858360e086016121c2565b61022083013581526102408301356020820152611ca360408201868360e086016121c2565b610e1b8160a0840160e0850180612100565b604051602081015160408201516060830151600084015160008051602061261483398151915284610260880135096000805160206126148339815191526101e0880135860960008051602061261483398151915261018089013582089050600080516020612614833981519152858208905060008051602061261483398151915261020089013587096000805160206126148339815191526101a08a01358208905060008051602061261483398151915286820890506000805160206126148339815191528284096000805160206126148339815191528282099050600080516020612614833981519152858209905060008051602061261483398151915260058009600080516020612614833981519152878a0998506000805160206126148339815191526101808c01358a089450600080516020612614833981519152888608945060008051602061261483398151915260058a0993506000805160206126148339815191526101a08c0135850893506000805160206126148339815191528885089350600080516020612614833981519152818a099250506000805160206126148339815191526101c08b013583089150600080516020612614833981519152878308915060008051602061261483398151915283850997506000805160206126148339815191528289096000805160206126148339815191529081039850858909975060008051602061261483398151915260808a01518908975061029488828c6119d6565b604051600263020000000161024060405101611f008183606086015161227e565b9150611f158183610140870160a08701612197565b611f2881610100860160a0860180612136565b611f37818360a086018061216c565b611f498160c0860160a0860180612136565b611f60816101c085015160a0860160a0870161216c565b505060c00180517f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd4703905250565b604051600060008051602061261483398151915260208301516101e085013509905060008051602061261483398151915260408301518208905060008051602061261483398151915261018084013582089050600060008051602061261483398151915260208401516102008601350990506000805160206126148339815191526040840151820890506000805160206126148339815191526101a085013582089050600060008051602061261483398151915260408501516101c08701350890506000805160206126148339815191528284099250600080516020612614833981519152818409925050600080516020612614833981519152600084015183099150600080516020612614833981519152610260850135830991506000805160206126148339815191526101a0840151830860808401519092506000805160206126148339815191529081039150818308600080516020612614833981519152036101209390930192909252505050565b8151845260208201516020850152825160408501526020830151606085015260408160808660065afa80610e1b57610e1b610463565b8151845260208201516020850152823560408501526020830135606085015260408160808660065afa80610e1b57610e1b610463565b815184526020820151602085015282604085015260408160608660075afa80610e1b57610e1b610463565b813584526020820135602085015282604085015260408160608660075afa80610e1b57610e1b610463565b815184526020820151602085015282604085015260408460608660075afa815160408601526020820151606086015260408260808760065afa1680610e1b57610e1b610463565b813584526020808301359085015260408481018490528460608160075afa815160408601526020820151606086015260408260808760065afa1680610e1b57610e1b610463565b6000805160206126148339815191528383350960008051602061261483398151915281835108825250505050565b6020835260208084015260206040840152806060840152508060808301525060008051602061261483398151915260a0820152600060208260c08460055afa50505190565b949350505050565b60008083601f8401126122dd57600080fd5b50813567ffffffffffffffff8111156122f557600080fd5b60208301915083602082850101111561230d57600080fd5b9250929050565b60008060008060006060868803121561232c57600080fd5b85359450602086013567ffffffffffffffff8082111561234b57600080fd5b61235789838a016122cb565b9096509450604088013591508082111561237057600080fd5b5061237d888289016122cb565b969995985093965092949392505050565b600080602083850312156123a157600080fd5b823567ffffffffffffffff8111156123b857600080fd5b6123c4858286016122cb565b90969095509350505050565b600080600080604085870312156123e657600080fd5b843567ffffffffffffffff808211156123fe57600080fd5b61240a888389016122cb565b9096509450602087013591508082111561242357600080fd5b818701915087601f83011261243757600080fd5b81358181111561244657600080fd5b8860208260051b850101111561245b57600080fd5b95989497505060200194505050565b600060208083528351808285015260005b818110156124975785810183015185820160400152820161247b565b506000604082860101526040601f19601f8301168501019250505092915050565b600080858511156124c857600080fd5b838611156124d557600080fd5b5050820193919092039150565b6001600160e01b0319813581811691600485101561250a5780818660040360031b1b83161692505b505092915050565b634e487b7160e01b600052603260045260246000fd5b60408152826040820152828460608301376000606084830101526000601f19601f8501168201606081016020606085840301818601528186518084526080850191508288019450600093505b808410156125945784518252938201936001939093019290820190612574565b5098975050505050505050565b6000602082840312156125b357600080fd5b815180151581146125c357600080fd5b9392505050565b8183823760009101908152919050565b6000602082840312156125ec57600080fd5b505191905056fe2a734ebb326341efa19b0361d9130cd47b26b7488dc6d26eeccd4f3eb878331a30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f000000130644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000000a26469706673582212205be76de136823c9e529b9ed54d489f0e1001a65cc1622c2acc3ed7067fb1ecd764736f6c63430008140033";