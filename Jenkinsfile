pipeline {
    agent any

    stages {
        stage('Deploy') {
            when {
                expression { env.BRANCH_NAME == "master" }
            }
            steps {
                dir('.') {
                    sshagent (credentials: ['jenkins-ssh-nfs-matt']) {
                        sh 'rsync -avrz -e "ssh -l mdibello_splendorveritatis -o StrictHostKeyChecking=no" . ssh.phx.nearlyfreespeech.net:/home/public/mozaa'
                    }
                }
            }
        }
    }
}