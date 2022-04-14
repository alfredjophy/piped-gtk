# define constants for now
import requests

class PIPED : 
    def __init__(self,instanceURL='https://pipedapi.kavin.rocks'):
        self.instanceURL = instanceURL
    
    def changeInstance(self,instanceURL):
        self.instanceURL = instanceURL

    def getInstanceURL(self):
        return self.instanceURL

    def getInstances(self):
        response = requests.get("https://raw.githubusercontent.com/wiki/TeamPiped/Piped-Frontend/Instances.md")
        responseText = response.text
        lines = responseText.split('\n')
        def parseLine(line):
            split = line.split("|")
            if len(split)  > 4 :
                instance = {
                    "name" : split[0].strip(),
                    "apiUrl" : split[1].strip(),
                    "locations":split[2].strip().split(','),
                    "cdn":split[3].strip() == 'Yes'
                }  
                return instance

        parsedInstances = list(map(parseLine,lines))
        instances = list(filter(lambda x: x != None,parsedInstances))
        return instances
            
    # instance API calls
    def streams(self,videoID):
        response = requests.get(self.instanceURL+'/streams/{0}'.format(videoID))
        return response.json()

    def comments(self,videoID):
        response = requests.get(self.instanceURL+'/comments/{0}'.format(videoID))
        return response.json()

    def commentsNext(self,videoID):
        response = requests.get(self.instanceURL+'/nextpage/comments/{0}'.format(videoID))
        return response.json()

    def trending(self,region='US'):
        response = requests.get(self.instanceURL+'/trending',params={'region': region})
        return response.json()
