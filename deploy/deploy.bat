set ip=10.0.0.40

docker save crabgame:latest > ./crabgame.tar

scp crabgame.tar matthewweisfeld@%ip%:~/Documents/G3Tech/Server/CrabGame

scp docker-compose.yaml matthewweisfeld@%ip%:~/Documents/G3Tech/Server/CrabGame
scp deploy/remoteDeploy.sh matthewweisfeld@%ip%:~/Documents/G3Tech/Server/CrabGame

ssh matthewweisfeld@%ip% chmod +x ~/Documents/G3Tech/Server/CrabGame/remoteDeploy.sh
ssh matthewweisfeld@%ip% ~/Documents/G3Tech/Server/CrabGame/remoteDeploy.sh