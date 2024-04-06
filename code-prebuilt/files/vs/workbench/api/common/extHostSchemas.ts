/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

import { createDecorator } from 'vs/platform/instantiation/common/instantiation';
import { ExtHostSchemasShape, MainContext, type MainThreadSchemasShape } from '../common/extHost.protocol';
import type { IJSONSchema } from 'vs/base/common/jsonSchema';
import { IExtHostRpcService } from 'vs/workbench/api/common/extHostRpcService';
import { ILogService } from 'vs/platform/log/common/log';

export interface IExtHostSchemas extends ExtHostSchemasShape {
	getSchemas(): Promise<{ [id: string]: IJSONSchema }>;
}

export const IExtHostSchemas = createDecorator<IExtHostSchemas>('IExtHostSchemas');

export class ExtHostSchemas implements ExtHostSchemasShape {

	protected readonly _proxy: MainThreadSchemasShape = this.extHostRpc.getProxy(MainContext.MainThreadSchemas);

	constructor(
		@IExtHostRpcService private extHostRpc: IExtHostRpcService,
		@ILogService protected _logService: ILogService
	) { }

	getSchemas(): Promise<{ [id: string]: IJSONSchema }> {
		return this._proxy.$getSchemas();
	}

	$provideSchemas(): Promise<{ [id: string]: IJSONSchema }> {
		return Promise.resolve({});
	}
}
