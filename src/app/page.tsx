"use client";

import { useEffect, useState } from "react";
import { X, Check } from "@geist-ui/icons";
import { invoke } from "@tauri-apps/api/core";
import { appDataDir } from "@tauri-apps/api/path";
import { open } from "@tauri-apps/plugin-dialog";

interface MenuButtonProps {
  title: string;
  action: () => void;
}

interface VaultViewModel {
  name: string;
  path: string;
  isLocked: boolean;
}

export default function Home() {
  const [vaultsShown, setVaultsShown] = useState(false);
  const [credentialScreenShown, setCredentialScreenShown] = useState(false);

  // User entered directory, name and password for the vault to be created
  const [userVaultDir, setUserVaultDir] = useState("");
  const [userVaultPassword, setUserVaultPassword] = useState("");
  const [userVaultName, setUserVaultName] = useState("");

  const getViewModel = async () => {
    let viewmodel: VaultViewModel = await invoke("get_view_model");
    console.log(viewmodel);
  };

  const buttons: MenuButtonProps[] = [
    {
      title: "Create new vault",
      action: async () => {
        const dir = await open({
          multiple: false,
          directory: true,
        });

        if (dir) {
          setUserVaultDir(dir);
        }

        setCredentialScreenShown(true);

        console.log("Created new vault!");
      },
    },
    {
      title: "Open a vault",
      action: () => {
        console.log("Opened a vault!");
        setVaultsShown(!vaultsShown);
      },
    },
    {
      title: "Delete a vault",
      action: async () => {
        await getViewModel();
      },
    },
  ];
  //border-b border-white/10
  return (
    <div className="relative h-screen">
      {/* Enter vault name & password */}
      {credentialScreenShown && (
        <div className="absolute top-0 left-0 w-full h-full flex justify-center items-center z-10 bg-white/10">
          <form className="flex flex-col relative w-[400px] h-[250px] bg-black rounded-lg">
            <div className="flex flex-col top-4 left-4 pt-4 px-4">
              <h1 className="text-2xl font-bold">Name</h1>
              <input
                className="shadow appearance-none border border-white/20 bg-black rounded w-full py-2 px-3 text-white leading-tight focus:outline-none focus:shadow-outline text-xl mt-2"
                id="username"
                type="text"
                placeholder="Name for the vault"
                onChange={(e) => setUserVaultName(e.target.value)}
              />
            </div>
            <div className="flex flex-col top-4 left-4 pt-4 px-4">
              <h1 className="text-2xl font-bold">Password</h1>
              <input
                className="shadow appearance-none border border-white/20 bg-black rounded w-full py-2 px-3 text-white leading-tight focus:outline-none focus:shadow-outline text-xl mt-2"
                id="password"
                type="password"
                placeholder="Password"
                onChange={(e) => setUserVaultPassword(e.target.value)}
              />
            </div>
            <div className="flex flex-row justify-end w-full py-2 px-4">
              <button
                className="hover:bg-white/10 rounded"
                onClick={async () => {
                  // const password = document.getElementById("password")?.value; // Fix this
                  // if (password) {
                  //   await invoke("create_secure_vault", {
                  //     path: userVaultDir,
                  //     password: password,
                  //   });
                  // }
                  await invoke("create_secure_vault", {
                    name: userVaultName,
                    path: userVaultDir,
                    password: userVaultPassword,
                  });
                  setCredentialScreenShown(false);
                }}
              >
                <Check size={32} />
              </button>
              <button
                className="hover:bg-white/10 rounded"
                onClick={() => setCredentialScreenShown(false)}
              >
                <X size={32} />
              </button>
            </div>
          </form>
        </div>
      )}
      {/* Modal */}
      {vaultsShown && <SelectVaults closeFunc={() => setVaultsShown(false)} />}

      {/* {vaultsShown && (
        <div className="absolute top-0 left-0 w-full h-full flex justify-center items-center z-10">
          <div className="relative w-[700px] h-[500px] bg-black">
            <div className="flex flex-row absolute inset-x-0 top-0 h-16">
              <div className="absolute top-4 left-4 px-4 py-2 flex justify-between w-full font-bold text-lg">
                <div className="flex">
                  <button onClick={() => setVaultsShown(false)}>
                    <X />
                  </button>
                  <h1 className="px-16">Vault</h1>
                </div>
                <div className="flex px-4">
                  <h1 className="px-4">Created</h1>
                  <h1 className="px-4">Last accessed</h1>
                </div>
              </div>
            </div>
          </div>
        </div>
      )} */}

      {/* Main View */}
      <div
        className={`flex flex-col items-center justify-between h-screen py-20 ${
          vaultsShown ? "bg-white/10" : ""
        } z-0`}
      >
        <div className="flex flex-col items-center">
          <h1 className="text-6xl text-white font-bold">
            Welcome to SecureMe.
          </h1>
          <p className="text-white">
            Please start by selecting an option below.
          </p>
        </div>
        <div className="flex flex-col space-y-2">
          {buttons.map(({ title, action }, index) => {
            return (
              <button
                className="border-0 rounded-lg bg-white/20"
                onClick={action}
                key={index}
              >
                <div className="my-2 mx-12 text-white">{title}</div>
              </button>
            );
          })}
        </div>
      </div>
    </div>
  );
}

// The modal section
{
  /* <div className="flex justify-between items-center px-4 py-4 border-b border-white/10">
  <div className="flex items-center space-x-4">
    <button
      className="p-2 rounded hover:bg-white/10"
      onClick={() => setVaultsShown(false)}
      aria-label="Close vaults modal"
    >
      <X />
    </button>
    <h1 className="font-bold text-lg">Vault</h1>
  </div>
  <div className="flex space-x-4 text-lg font-bold">
    <span>Created</span>
    <span>Last accessed</span>
  </div>
</div>; */
}

const SelectVaults = ({ closeFunc }: { closeFunc: () => void }) => {
  const [vaults, setVaults] = useState([]);

  useEffect(() => {
    const getVaults = async () => {
      let vaults: VaultViewModel[] = await invoke("get_vaults");
      setVaults(vaults);
      console.log(vaults);
    };

    getVaults();
  }, []);

  return (
    <div className="absolute top-0 left-0 w-full h-full flex justify-center items-center z-10 bg-white/10">
      <div className="relative w-[700px] h-[500px] bg-black text-white rounded-lg">
        {/* Header Section */}
        <div className="flex justify-between items-center px-4 py-4 border-b border-white/10">
          <div className="flex items-center space-x-4">
            <button
              className="p-2 rounded hover:bg-white/10"
              onClick={() => closeFunc()}
              aria-label="Close vaults modal"
            >
              <X />
            </button>
            <h1 className="font-bold text-lg">Vault</h1>
          </div>
          <div className="flex space-x-4 text-lg font-bold">
            <span>Created</span>
            <span>Last accessed</span>
          </div>
        </div>

        {/* Modal Content */}
        <div className="p-4 overflow-y-auto h-[calc(100%-4rem)]">
          <p className="text-center text-gray-400">Your vault content...</p>
        </div>
      </div>
    </div>
  );
};
