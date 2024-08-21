每日定时推送

华为云 - 函数工作流（每月有免费额度

1.创建函数

![QQ_1724211106790](https://github.com/user-attachments/assets/56cc130f-c391-459b-a5fe-5861df350db8)

2.代码

![QQ_1724211440805](https://github.com/user-attachments/assets/11561414-a679-4733-b833-0d4ec68e7d8d)

import json

import requests

import os

def handler (event, context):

    url = f"https://api.github.com/repos/github用户名/仓库名/actions/workflows/workflow文件名/dispatches"
    
    headers = {
    
        "Authorization": f"token {os.getenv('GITHUB_TOKEN')}",
        
        "Accept": "application/vnd.github.everest-preview+json",
        
    }
    
    data = {
    
        "ref": "分支名"
        
    }
    
    response = requests.post(url, headers=headers, data=json.dumps(data))
    
    return {
    
        "statusCode": response.status_code,
        
        "isBase64Encoded": False,
        
        "body": response.text,
        
        "headers": {
        
            "Content-Type": "application/json"
            
        }
        
    }
    
3.环境变量

![QQ_1724211194422](https://github.com/user-attachments/assets/9415865d-ff32-4fb6-ad9b-9a97f13421bb)

4.设置定时触发器

![QQ_1724211249821](https://github.com/user-attachments/assets/9ba1a372-f8a1-44b2-a02a-4c10d3c86cda)

cron表达式

     CRON_TZ=Asia/Shanghai 0 59 09 * * *

                             分 时
                        
5.测试

![QQ_1724211339157](https://github.com/user-attachments/assets/bc7032dc-e117-4eac-b6f7-dbd1f1b1d373)
