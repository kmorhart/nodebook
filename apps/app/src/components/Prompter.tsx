import styles from './Prompter.module.css'
import * as Icons from '../components/icons'

export default function Prompter() {
    return (
        <div className={styles.prompter}>
            <textarea id="prompt" className={styles.prompt} placeholder="Type your prompt here..." />
            <button className={styles['prompt-button']}>
                <Icons.Send />
            </button>
        </div>
    )
}