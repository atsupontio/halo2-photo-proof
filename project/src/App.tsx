import { FormEvent, useCallback, useEffect, useRef, useState } from "react";
import init, {exec_mosaic, create_proof, verify_proof } from "./pkg/wasm";

function App() {
  const [loadWasm, setLoadWasmFlg] = useState(false);
  const [loadedImage, setImage] = useState<HTMLImageElement | null>(null);
  const [loadedSmallImage, setLoadedSmallImage] = useState<ArrayBuffer | null>(null);
  // const [loadedSmnallImage, setSmallImage] = useState<HTMLImageElement | null>(null);
  const [grain, setGrain] = useState(0);
  const [proof, setProof] = useState<ArrayBuffer | null>(null);

  const rawImagecanvasRef = useRef<HTMLCanvasElement>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    init()
      .then(() => {
        console.log("set wasm is successfully.");
        setLoadWasmFlg(true);
      })
      .then((err) => {
        console.error("err", err);
      });
  });

  useEffect(() => {
    if (!loadWasm) return;
  }, [loadWasm]);

  const handleGenerateProof = () => {
    const imageBuf = getImageData();
    console.log("loadedImage", loadedImage?.width, loadedImage?.height);
    if (imageBuf) {
      const startTime = performance.now();
      console.log("start");
      // create_proof 関数に渡す
      const proof = create_proof(
        imageBuf,
        loadedImage?.width || 0, // 幅
        loadedImage?.height || 0, // 高さ
      );
      const endTime = performance.now();
      console.log(endTime - startTime); 
      // return <ByteDataDisplay byteData={proof} />;
      setProof(proof);
    
    } else {
      // エラーハンドリング（getImageData が undefined を返した場合の処理）
      console.error("Image data is undefined");
    }
  }

  // バイト列をBase64エンコードする関数
  function arrayBufferToBase64(buffer: ArrayBuffer) {
    const bytes = new Uint8Array(buffer);
    let binary = '';
    for (let i = 0; i < bytes.byteLength; i++) {
      binary += String.fromCharCode(bytes[i]);
    }
    return window.btoa(binary);
  }

  // バイト列を表示するコンポーネント
  function ByteDataDisplay({ byteData }: { byteData: ArrayBuffer | null }) {
    if (!byteData) {
      return null; // byteDataがnullの場合は何も表示しない
    }
    const base64Data = arrayBufferToBase64(byteData);
    const dataUrl = `data:application/octet-stream;base64,${base64Data}`;

    return <img src={dataUrl} alt="Byte Data" />;
  }


  const getImageData = (): Uint8ClampedArray | undefined => {
    if (!loadedImage) {
      console.error("Image not loaded");
      return;
    }

    const canvas = document.createElement("canvas");
    canvas.width = loadedImage.width;
    canvas.height = loadedImage.height;
    const ctx = canvas.getContext("2d");

    if (!ctx) {
      console.error("Canvas context not available");
      return;
    }

    // 画像を Canvas に描画
    ctx.drawImage(loadedImage, 0, 0);

    // Canvas から ImageData を取得
    const imageData = ctx.getImageData(0, 0, loadedImage.width, loadedImage.height);

    // ImageData から Uint8ClampedArray を取得
    const uint8ClampedArray = new Uint8ClampedArray(imageData.data.buffer);

    // uint8ClampedArray を返す
    return uint8ClampedArray;
  };

  const handleVerifyProof = () => {
    const imageBuf = loadedSmallImage;

    if (imageBuf && proof) {
      // ImageData から Uint8ClampedArray を取得
      const proofArray = new Uint8ClampedArray(proof);
      const smallImageArray = new Uint8ClampedArray(imageBuf);
      console.log(loadedImage?.height ? Math.floor(loadedImage.height / 2) : 0)
      // create_proof 関数に渡す
      const result = verify_proof(
        proofArray,
        smallImageArray,
        loadedImage?.width ? Math.floor(loadedImage.width / 2) : 0, // 幅
        loadedImage?.height ? Math.floor(loadedImage.height / 2) : 0, // 高さ
      );

      console.log("result", result);
    
    } else {
      // エラーハンドリング（getImageData が undefined を返した場合の処理）
      console.error("Image data is undefined");
    }
  }


  const handleSaveImage = () => {
    if (!canvasRef.current) {
      console.error("Canvas element not found");
      return;
    }

    // Canvas要素からデータURLを取得
    const dataURL = canvasRef.current.toDataURL("image/png"); // ここではPNG形式を使用

    // データURLをファイルに変換
    const blob = dataURLToBlob(dataURL);

    // ファイルを保存
    saveBlobAsFile(blob, "image.jpg");
  };

  // データURLをBlobに変換する関数
  const dataURLToBlob = (dataURL: string): Blob => {
    const parts = dataURL.split(",");
    const matchResult = parts[0].match(/:(.*?);/);
    const contentType = matchResult ? matchResult[1] : "";
    const byteString = atob(parts[1]);
    const arrayBuffer = new ArrayBuffer(byteString.length);
    const uint8Array = new Uint8Array(arrayBuffer);

    for (let i = 0; i < byteString.length; i++) {
      uint8Array[i] = byteString.charCodeAt(i);
    }

    return new Blob([arrayBuffer], { type: contentType });
  };

  // Blobをファイルとして保存する関数
  const saveBlobAsFile = (blob: Blob, fileName: string) => {
    const link = document.createElement("a");
    link.href = window.URL.createObjectURL(blob);
    link.download = fileName;
    link.click();
  };

  const handleSubmit = useCallback((e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    setImage(null);
    const image = new Image();
    const inputForm = e.target;
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    const grain = Number((inputForm["grain"] as HTMLInputElement).value);
    setGrain(grain);
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    const fileInputEl = inputForm["file"];
    const [file] = fileInputEl.files as FileList;
    if (!file) {
      alert("ファイルが選択されていません。");
      return;
    }


    const reader = new FileReader();
    reader.readAsDataURL(file);
    reader.addEventListener("load", (event) => {
      const imageUrl = event.target?.result;
      // When array buffer comes, you should read image as readAsDataURL not readAsArrayBuffer.
      if (typeof imageUrl !== "string") {
        alert("画像の読み込みに失敗しました。");
        return;
      }
      image.src = imageUrl;
    });
    image.onload = function () {
      console.log("Image loaded successfully.");
      setImage(image);
    };
  }, []);

  useEffect(() => {
    if (!loadedImage || !loadWasm) return;
    const canvasRenderingContext = rawImagecanvasRef.current?.getContext("2d");
    if (!canvasRenderingContext) {
      alert("Not found canvas el");
      return;
    }
    canvasRenderingContext.drawImage(
      loadedImage,
      0,
      0,
      loadedImage.width,
      loadedImage.height
    );
    const imageData = canvasRenderingContext.getImageData(
      0,
      0,
      loadedImage.width,
      loadedImage.height
    );
    rawImagecanvasRef.current?.getContext("2d")?.putImageData(imageData, 0, 0);
    console.log("loadedimage", loadedImage.width, loadedImage.height)
    const new_width = Math.floor(loadedImage.width / 2);
    const new_height = Math.floor(loadedImage.height / 2);
    console.log("loadedimage", Math.floor(loadedImage.width / 2), Math.floor(loadedImage.height / 2))


    const mosaiced = exec_mosaic(
      imageData.data,
      grain,
      loadedImage.width,
      loadedImage.height,
    );

    const iamgedata = new ImageData(
      new Uint8ClampedArray(mosaiced.buffer),
      new_width,
      new_height
    );
    setLoadedSmallImage(iamgedata.data);
    console.log("iamgedata", iamgedata.width, iamgedata.height);
    const canvasRefCurrent = canvasRef.current; // null チェックのために変数に格納

    if (canvasRefCurrent) {
      const canvasRenderingContextResult = canvasRefCurrent.getContext("2d");
      if (canvasRenderingContextResult) {
        // canvasRenderingContextResult.clearRect(0, 0, canvasRefCurrent.width, canvasRefCurrent.height);
        canvasRenderingContextResult.putImageData(iamgedata, 0, 0, 0, 0, new_width, new_height);
      } else {
        alert("Not found canvas el for result");
      }
    } else {
      alert("canvasRef is null");
    }
  }, [loadedImage, loadWasm]);

  return (
    <div className="App">
      <h1>mosaic</h1>
      <p>Online mosaic tool.</p>
      <p>
        Tech stack is{" "}
        <a href="https://blog.ojisan.io/rust-mosaic-web-app/" target="_brank">
          here
        </a>
      </p>
      <form onSubmit={handleSubmit}>
        <label htmlFor="grain-input">Grain</label>
        <input
          name="grain"
          type="number"
          min="0"
          id="grain-input"
          defaultValue={2}
          required
        ></input>
        <label htmlFor="file-input">Image</label>
        <input type="file" name="file" id="file-input" required></input>
        <br />
        <button type="submit">Run</button>
      </form>
      <canvas
        ref={rawImagecanvasRef}
        width={loadedImage?.width}
        height={loadedImage?.height}
        style={{ maxWidth: "100%", maxHeight: "400px" }}
      ></canvas>
      <canvas
        ref={canvasRef}
        width={loadedImage?.width ? Math.floor(loadedImage.width / 2) : undefined} // 幅を半分に設定
        height={loadedImage?.height ? Math.floor(loadedImage.height / 2) : undefined}
        style={{ maxWidth: "100%", maxHeight: "400px" }}
      ></canvas>
      <p>
      <button onClick={handleSaveImage}>Save Image</button>
      <button onClick={handleGenerateProof}>Generate Proof</button>
      <button onClick={handleVerifyProof}>Verify Proof</button>
      </p>
      {/* <p>JsValueの文字列表現: {proof}</p> */}
      <p>proof: <ByteDataDisplay byteData={proof} /></p>
      {/* <button onClick={handleVerifyProof}>Verify Proof</button> */}
    </div>
  );
}

export default App;