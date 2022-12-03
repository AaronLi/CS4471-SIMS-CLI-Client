from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class ActionApproved(_message.Message):
    __slots__ = []
    def __init__(self) -> None: ...

class ClientAction(_message.Message):
    __slots__ = ["action", "info", "user_id"]
    ACTION_FIELD_NUMBER: _ClassVar[int]
    INFO_FIELD_NUMBER: _ClassVar[int]
    USER_ID_FIELD_NUMBER: _ClassVar[int]
    action: str
    info: Info
    user_id: str
    def __init__(self, user_id: _Optional[str] = ..., action: _Optional[str] = ..., info: _Optional[_Union[Info, _Mapping]] = ...) -> None: ...

class CreateShelfRequest(_message.Message):
    __slots__ = ["shelfinfo", "token", "username"]
    SHELFINFO_FIELD_NUMBER: _ClassVar[int]
    TOKEN_FIELD_NUMBER: _ClassVar[int]
    USERNAME_FIELD_NUMBER: _ClassVar[int]
    shelfinfo: ShelfInfo
    token: str
    username: str
    def __init__(self, username: _Optional[str] = ..., token: _Optional[str] = ..., shelfinfo: _Optional[_Union[ShelfInfo, _Mapping]] = ...) -> None: ...

class GetItemRequest(_message.Message):
    __slots__ = ["itemID", "token", "username"]
    ITEMID_FIELD_NUMBER: _ClassVar[int]
    TOKEN_FIELD_NUMBER: _ClassVar[int]
    USERNAME_FIELD_NUMBER: _ClassVar[int]
    itemID: int
    token: str
    username: str
    def __init__(self, username: _Optional[str] = ..., token: _Optional[str] = ..., itemID: _Optional[int] = ...) -> None: ...

class GetItemsRequest(_message.Message):
    __slots__ = ["shelf_id", "token", "username"]
    SHELF_ID_FIELD_NUMBER: _ClassVar[int]
    TOKEN_FIELD_NUMBER: _ClassVar[int]
    USERNAME_FIELD_NUMBER: _ClassVar[int]
    shelf_id: str
    token: str
    username: str
    def __init__(self, shelf_id: _Optional[str] = ..., username: _Optional[str] = ..., token: _Optional[str] = ...) -> None: ...

class GetShelvesRequest(_message.Message):
    __slots__ = ["shelf_id", "token", "username"]
    SHELF_ID_FIELD_NUMBER: _ClassVar[int]
    TOKEN_FIELD_NUMBER: _ClassVar[int]
    USERNAME_FIELD_NUMBER: _ClassVar[int]
    shelf_id: str
    token: str
    username: str
    def __init__(self, shelf_id: _Optional[str] = ..., username: _Optional[str] = ..., token: _Optional[str] = ...) -> None: ...

class GetSlotRequest(_message.Message):
    __slots__ = ["slot_num", "token", "username"]
    SLOT_NUM_FIELD_NUMBER: _ClassVar[int]
    TOKEN_FIELD_NUMBER: _ClassVar[int]
    USERNAME_FIELD_NUMBER: _ClassVar[int]
    slot_num: str
    token: str
    username: str
    def __init__(self, slot_num: _Optional[str] = ..., username: _Optional[str] = ..., token: _Optional[str] = ...) -> None: ...

class Info(_message.Message):
    __slots__ = ["itemInfo", "shelfInfo", "slotInfo"]
    ITEMINFO_FIELD_NUMBER: _ClassVar[int]
    SHELFINFO_FIELD_NUMBER: _ClassVar[int]
    SLOTINFO_FIELD_NUMBER: _ClassVar[int]
    itemInfo: ItemInfo
    shelfInfo: ShelfInfo
    slotInfo: SlotInfo
    def __init__(self, shelfInfo: _Optional[_Union[ShelfInfo, _Mapping]] = ..., itemInfo: _Optional[_Union[ItemInfo, _Mapping]] = ..., slotInfo: _Optional[_Union[SlotInfo, _Mapping]] = ...) -> None: ...

class ItemInfo(_message.Message):
    __slots__ = ["description", "object_id", "price", "shelf_id", "stock"]
    DESCRIPTION_FIELD_NUMBER: _ClassVar[int]
    OBJECT_ID_FIELD_NUMBER: _ClassVar[int]
    PRICE_FIELD_NUMBER: _ClassVar[int]
    SHELF_ID_FIELD_NUMBER: _ClassVar[int]
    STOCK_FIELD_NUMBER: _ClassVar[int]
    description: str
    object_id: int
    price: int
    shelf_id: str
    stock: int
    def __init__(self, description: _Optional[str] = ..., object_id: _Optional[int] = ..., shelf_id: _Optional[str] = ..., price: _Optional[int] = ..., stock: _Optional[int] = ...) -> None: ...

class Items(_message.Message):
    __slots__ = ["items"]
    ITEMS_FIELD_NUMBER: _ClassVar[int]
    items: _containers.RepeatedCompositeFieldContainer[ItemInfo]
    def __init__(self, items: _Optional[_Iterable[_Union[ItemInfo, _Mapping]]] = ...) -> None: ...

class LoginRequest(_message.Message):
    __slots__ = ["password", "username"]
    PASSWORD_FIELD_NUMBER: _ClassVar[int]
    USERNAME_FIELD_NUMBER: _ClassVar[int]
    password: str
    username: str
    def __init__(self, username: _Optional[str] = ..., password: _Optional[str] = ...) -> None: ...

class ShelfInfo(_message.Message):
    __slots__ = ["shelf_count", "shelf_id"]
    SHELF_COUNT_FIELD_NUMBER: _ClassVar[int]
    SHELF_ID_FIELD_NUMBER: _ClassVar[int]
    shelf_count: int
    shelf_id: str
    def __init__(self, shelf_id: _Optional[str] = ..., shelf_count: _Optional[int] = ...) -> None: ...

class Shelves(_message.Message):
    __slots__ = ["shelves"]
    SHELVES_FIELD_NUMBER: _ClassVar[int]
    shelves: _containers.RepeatedCompositeFieldContainer[ShelfInfo]
    def __init__(self, shelves: _Optional[_Iterable[_Union[ShelfInfo, _Mapping]]] = ...) -> None: ...

class SlotInfo(_message.Message):
    __slots__ = ["capacity", "item_count", "slot_num"]
    CAPACITY_FIELD_NUMBER: _ClassVar[int]
    ITEM_COUNT_FIELD_NUMBER: _ClassVar[int]
    SLOT_NUM_FIELD_NUMBER: _ClassVar[int]
    capacity: int
    item_count: int
    slot_num: int
    def __init__(self, slot_num: _Optional[int] = ..., capacity: _Optional[int] = ..., item_count: _Optional[int] = ...) -> None: ...

class Slots(_message.Message):
    __slots__ = ["slots"]
    SLOTS_FIELD_NUMBER: _ClassVar[int]
    slots: _containers.RepeatedCompositeFieldContainer[SlotInfo]
    def __init__(self, slots: _Optional[_Iterable[_Union[SlotInfo, _Mapping]]] = ...) -> None: ...

class Token(_message.Message):
    __slots__ = ["token"]
    TOKEN_FIELD_NUMBER: _ClassVar[int]
    token: str
    def __init__(self, token: _Optional[str] = ...) -> None: ...
