import init, { Tetris } from 'tetris'
import { useState, useEffect, Suspense } from 'react'

export default function TetrisComponent() {
  const [game, setGame] = useState<any>(null)
  const [gameState, setGameState] = useState<any>(null)
  useEffect(() => {
    init().then(() => setGame(new Tetris(10, 20)))
  }, [])

  useEffect(() => {
    if (!game) return
    setTimeout(() => {
      setGameState(game.tick())
    }, 1000)
  }, [game])

  if (!game || !gameState) return <p>loading...</p>

  return <Suspense fallback={<p>loading...</p>}>
    <p>Game: {game.toString()}</p>
    <p>GameState: {gameState.toString()}</p>
  </Suspense>
}
