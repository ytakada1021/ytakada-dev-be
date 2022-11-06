import { LambdaIntegration, RestApi } from '@aws-cdk/aws-apigateway';
import { Code, Function, Runtime } from '@aws-cdk/aws-lambda';
import { Construct, Duration, Stack, StackProps } from '@aws-cdk/core';

export class LambdaStack extends Stack {
    constructor(scope: Construct, id: string, props?: StackProps) {
        super(scope, id, props);

        const target = process.env.TARGET as string;

        const savePostFunction = new Function(
            this,
            'savePostFunction',
            {
                functionName: 'savePostFunction',
                runtime: Runtime.PROVIDED_AL2,
                handler: 'handler',
                code: Code.fromAsset(`${__dirname}/../../target/${target}/deploy/save-post`),
                timeout: Duration.seconds(10),
                environment: {
                    DB_URL: process.env.DB_URL as string,
                    DB_NAME: process.env.DB_NAME as string,
                    API_KEY: process.env.API_KEY as string,
                }
            }
        )

        // const deletePostFunction = new Function(
        //     this,
        //     'deletePostFunction',
        //     {
        //         functionName: 'deletePostFunction',
        //         runtime: Runtime.PROVIDED_AL2,
        //         handler: 'handler',
        //         code: Code.fromAsset(`${__dirname}/../../functions/delete-post/target/cdk/debug`),
        //         timeout: Duration.seconds(10)
        //     }
        // )

        // Define rest api.
        const api = new RestApi(
            this,
            'restApi',
            {
                restApiName: "ytakada.dev API"
            }
        );
        // Define resource.
        const posts = api.root.addResource('posts');
        const singlePost = posts.addResource('{id}');
        singlePost.addMethod("PUT", new LambdaIntegration(savePostFunction));
        // singlePost.addMethod('DELETE', new LambdaIntegration(deletePostFunction))
    }
}
