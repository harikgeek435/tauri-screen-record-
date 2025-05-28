'use client'
import { useState } from 'react'
import { invokeSafe } from '../utils/invokeSafe'

export default function Page() {
  const [recording, setRecording] = useState(false)
  const output = 'C:/Users/Public/recorded.mp4'

  const start = async () => {
    try {
      await invokeSafe('start_recording', { output })
      setRecording(true)
    } catch (err) {
      console.error('Start failed', err)
    }
  }

  const stop = async () => {
    try {
      await invokeSafe('stop_recording')
      setRecording(false)
    } catch (err) {
      console.error('Stop failed', err)
    }
  }

  return (
    <main style={{ padding: 20 }}>
      <h1>🎥 Screen Recorder</h1>
      <button onClick={start} disabled={recording} style={{ marginRight: 10 }}>
        Start Recording
      </button>
      <button onClick={stop} disabled={!recording}>
        Stop Recording
      </button>
    </main>
  )
}
