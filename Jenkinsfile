pipeline {
    agent any

    stages {
        stage('Build WASM') {
            steps {
                sh 'cargo build --release --target wasm32-unknown-unknown'
            }
        }
        stage('Generate JS') {
            steps {
                sh 'wasm-bindgen --target web ./target/wasm32-unknown-unknown/release/mozaa.wasm --out-dir ./pkg'
            }
        }
        stage('Deploy') {
            when {
                expression { env.BRANCH_NAME == "master" }
            }
            steps {
                dir('.') {
                    sshagent (credentials: ['jenkins-ssh-nfs-matt']) {
                        sh 'rsync -avrz -e "ssh -l mdibello_splendorveritatis -o StrictHostKeyChecking=no" index.html favicon.svg script.js style.css ssh.phx.nearlyfreespeech.net:/home/public/mozaa'
                        sh 'rsync -avrz -e "ssh -l mdibello_splendorveritatis -o StrictHostKeyChecking=no" pkg/ ssh.phx.nearlyfreespeech.net:/home/public/mozaa/pkg'
                    }
                }
            }
        }
    }
}