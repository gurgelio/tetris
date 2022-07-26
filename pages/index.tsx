import type { NextPage } from 'next'
import Head from 'next/head'
import TetrisComponent from '../components/tetris'
import styles from '../styles/Home.module.css'

const Home: NextPage = () => {
  return (
    <div className={styles.container}>
      <Head>
        <title>Tetris</title>
        <meta name="description" content="Tetris por gurgel.io" />
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <TetrisComponent />
    </div>
  )
}

export default Home
