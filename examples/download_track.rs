// Copyright (c) 2016, Mikkel Kroman <mk@uplink.io>
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
//
// * Redistributions of source code must retain the above copyright notice, this
//   list of conditions and the following disclaimer.
//
// * Redistributions in binary form must reproduce the above copyright notice,
//   this list of conditions and the following disclaimer in the documentation
//   and/or other materials provided with the distribution.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
// FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
// CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
// OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use tokio::fs::File;
use tokio_util::compat::Tokio02AsyncWriteCompatExt;

#[tokio::main]
async fn main()-> Result<(), Box<dyn std::error::Error>> {
    let soundcloud_client_id = env!("SOUNDCLOUD_CLIENT_ID");
    let client = soundcloud::Client::new(&soundcloud_client_id);
    let tracks = client.tracks().query(Some("montebooker")).get().await?;

    for track in &tracks {
        if !track.downloadable {
            continue;
        }
        let track_title = track.title.to_string()
            .replace(&['\"', '.', '\'', '\\', '/', '?', '*'][..], "");
        let path = format!("{}.{}", track_title, track.original_format.to_string());
        let mut outfile = File::create(&path).await?.compat_write();

        if let Ok(num_bytes) = client.download(track, &mut outfile).await {
            if num_bytes > 0 {
                break;
            }
        }
    }
    Ok(())
}
