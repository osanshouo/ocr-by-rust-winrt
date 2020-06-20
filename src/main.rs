winrt::import!(
    dependencies
        os
    types
        windows::storage::*
        windows::graphics::imaging::*
        windows::media::ocr::*
);

use std::env;
use windows::globalization::Language;
use windows::storage::{StorageFile, FileAccessMode};
use windows::graphics::imaging::{BitmapDecoder, SoftwareBitmap};
use windows::media::ocr::OcrEngine;

const LANG: &'static str = "ja";

fn open_image() -> winrt::Result<SoftwareBitmap> {
    // コマンドライン引数の取得
    let path = env::args().nth(1).expect("ファイルパスを引数に指定してください.");

    // ファイルパスから `StorageFile` オブジェクトを取得
    let file = StorageFile::get_file_from_path_async(path)?.get()?;

    // ファイルを読み込みビットマップに変換
    let bitmap = BitmapDecoder::create_with_id_async(
        BitmapDecoder::png_decoder_id()?, // ファイル形式は PNG とする
        file.open_async(FileAccessMode::Read)?.get()?
    )?.get()?;

    // `SoftwareBitmap` 型へ変換
    bitmap.get_software_bitmap_async()?.get()
}

fn ocr(bitmap: SoftwareBitmap) -> winrt::Result<()> {
    // 言語タグをもとに OCR エンジンを生成
    let lang = Language::create_language(LANG)?;
    let engine = OcrEngine::try_create_from_language(lang)?;

    // OCR の実行
    let result = engine.recognize_async(bitmap)?.get()?;

    // 結果を表示
    println!("{}", result.text()?);

    Ok(())
}

fn main() -> winrt::Result<()> {
    // 画像ファイルを開きビットマップに変換
    let bitmap = open_image()?;

    // OCR を実行
    ocr(bitmap)
}
