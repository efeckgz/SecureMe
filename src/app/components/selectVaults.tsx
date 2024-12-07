import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { X, Trash2, Lock, Unlock } from "@geist-ui/icons";

import Button from "../components/common/button";

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
            <Button onClick={() => closeFunc()}>
              <X />
            </Button>
            <h1 className="font-bold text-lg">Vault</h1>
          </div>
          <div className="flex space-x-4 text-lg font-bold">
            <span>Created</span>
            <span>Last accessed</span>
          </div>
        </div>
        {/* Modal Content */}
        <div className="overflow-y-auto h-[calc(97%-4rem)]">
          {/* <p className="text-center text-gray-400">Your vault content...</p> */}
          {vaults.map(({ name, path, isLocked }: VaultViewModel, key) => {
            return (
              <div
                className="flex justify-between items-center w-full px-4 py-4 border-b border-white/10 "
                key={key}
              >
                <div className="flex flex-row items-center space-x-4">
                  <Button onClick={() => {}}>
                    <Trash2 />
                  </Button>
                  <div className="flex flex-col">
                    <h1 className="font-bold text-lg">{name}</h1>
                    <h2 className="font-thin text-sm text-white/50">{path}</h2>
                  </div>
                </div>
                <Button onClick={async () => console.log("lock vault")}>
                  <Lock />
                </Button>
                <h2 className="font-thin text-sm text-white/50">
                  Creation date
                </h2>
                <h2 className="font-thin text-sm text-white/50">
                  Last accessed
                </h2>
              </div>
            );
          })}
        </div>
      </div>
    </div>
  );
};

export default SelectVaults;
