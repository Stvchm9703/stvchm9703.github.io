from dataclasses import dataclass

@dataclass
class Metadata:
    needs_background: str
    kernelspec: Kernelspec
    language_info: LanguageInfo
    orig_nbformat: int


@dataclass
class Cells:
    cell_type: str
    execution_count: str
    metadata: Metadata
    outputs: List
    source: List
    attachments: Attachments


@dataclass
class Attachments:


@dataclass
class Outputs:
    ename: str
    evalue: str
    output_type: str
    traceback: List[str]
    data: Data
    execution_count: int
    metadata: Metadata
    name: str
    text: List[str]


@dataclass
class Data:
    # text/plain
    text_plain: List[str]
    # text/html
    text_html: List[str]
    # image/png
    image_png: str


@dataclass
class Kernelspec:
    display_name: str
    language: str
    name: str


@dataclass
class CodemirrorMode:
    name: str
    version: int


@dataclass
class LanguageInfo:
    codemirror_mode: CodemirrorMode
    file_extension: str
    mimetype: str
    name: str
    nbconvert_exporter: str
    pygments_lexer: str
    version: str


@dataclass
class IpynbFile:
    cells: List[Cells]
    metadata: Metadata
    nbformat: int
    nbformat_minor: int

    def from_json(data: dict) -> 'IpynbFile':
        cells = [Cells(**cell) for cell in data['cells']]
        metadata = Metadata(**data['metadata'])
        nbformat = data['nbformat']
        nbformat_minor = data['nbformat_minor']
        return IpynbFile(cells, metadata, nbformat, nbformat_minor)

def load_ipynb_file(file_path: str) -> IpynbFile:
    with open(file_path, 'r') as file:
        data = json.load(file)
        return IpynbFile.from_json(data)
